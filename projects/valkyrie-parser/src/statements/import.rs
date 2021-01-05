use super::*;
use lispify::Lispify;

pub static NAMESPACE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
    namespace[*!?]?
)",
    )
    .unwrap()
});

impl ThisParser for NamespaceDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, ns) = input.match_regex(&NAMESPACE, "NAMESPACE")?;
        let (finally, names) = state.skip(ignore).match_fn(parse)?;
        let kind = match ns.as_str() {
            "namespace*" => NamespaceKind::Test,
            _ => NamespaceKind::Unique,
        };
        finally.finish(NamespaceDeclaration::new(names, get_span(input, finally)).with_kind(kind))
    }

    fn lispify(&self) -> Lisp {
        self.lispify()
    }
}

fn parse(input: ParseState) -> ParseResult<Vec<IdentifierNode>> {
    let mut names = Vec::new();
    let (state, id) = input.match_fn(IdentifierNode::parse)?;
    names.push(id);
    let (state, _) = state.match_repeats(|s| pare_colon_id(s, &mut names))?;
    state.finish(names)
}

fn pare_colon_id<'i>(input: ParseState<'i>, names: &mut Vec<IdentifierNode>) -> ParseResult<'i, ()> {
    let (state, _) = parse_name_join_dot(input)?;
    let (state, id) = state.match_fn(|s| IdentifierNode::parse(s))?;
    names.push(id);
    state.finish(())
}

impl ThisParser for ImportStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("using")?;
        let (state, head) = state.skip(ignore).match_fn(ImportTermNode::parse)?;
        state.finish(ImportStatement { annotation: AnnotationList::default(), term: head, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(2);
        lisp += Lisp::keyword("using");
        lisp += match &self.term {
            ImportTermNode::Alias(v) => v.lispify(),
            ImportTermNode::Group(v) => v.lispify(),
        };
        lisp
    }
}

impl ThisParser for ImportTermNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose(|s| ImportAliasNode::parse(s).map_into())
            .choose(|s| ImportGroupNode::parse(s).map_into())
            .end_choice()
    }

    fn lispify(&self) -> Lisp {
        match self {
            ImportTermNode::Alias(v) => v.lispify(),
            ImportTermNode::Group(v) => v.lispify(),
        }
    }
}

impl ThisParser for ImportAliasNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, path) = parse_any_name_path(input, parse_name_id)?;
        let (state, _) = state.skip(ignore).match_str("as")?;
        let (state, alias) = state.skip(ignore).match_fn(IdentifierNode::parse)?;
        state.finish(ImportAliasNode::new(path, alias))
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword("import/alias") + self.path.lispify() + self.alias.lispify()
    }
}

impl ThisParser for ImportGroupNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, path) = parse_any_name_path(input, parse_name_id)?;
        let (state, group) = state.match_optional(parse_maybe_group)?;
        state.finish(ImportGroupNode::new(path, group.unwrap_or_default()))
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(self.group.len() + 2);
        lisp += Lisp::keyword("import/group");
        lisp += self.path.lispify();
        for i in self.group.iter() {
            lisp += i.lispify();
        }
        lisp
    }
}

/// `.? { body }`
fn parse_maybe_group(input: ParseState) -> ParseResult<Vec<ImportTermNode>> {
    let (state, _) = input.skip(ignore).match_optional(parse_name_join_dot)?;
    let mut group = vec![];
    let (state, _) = parse_group_body(state, &mut group)?;
    state.finish(group)
}

fn parse_group_body<'a>(input: ParseState<'a>, items: &mut Vec<ImportTermNode>) -> ParseResult<'a, ()> {
    let (state, _) = input.skip(ignore).match_str("{")?;
    let (state, _) = state.match_repeats(|s| {
        let (state, term) = s.skip(ignore).match_fn(ImportTermNode::parse)?;
        items.push(term);
        parse_group_delimiter(state)
    })?;
    let (state, _) = state.skip(ignore).match_str("}")?;
    state.finish(())
}

fn parse_group_delimiter(input: ParseState) -> ParseResult<()> {
    let state = input.skip(ignore);
    if state.residual.starts_with(",") {
        state.advance(",").finish(())
    }
    else if state.residual.starts_with(";") {
        state.advance(";").finish(())
    }
    else {
        state.finish(())
    }
}

fn parse_name_id(input: ParseState) -> ParseResult<IdentifierNode> {
    input.begin_choice().choose(|s| IdentifierNode::parse(s)).choose(parse_start_as_name).end_choice()
}

fn parse_start_as_name(input: ParseState) -> ParseResult<IdentifierNode> {
    let (state, all) = input.match_str("*")?;
    state.finish(IdentifierNode::new(all, get_span(input, state)))
}

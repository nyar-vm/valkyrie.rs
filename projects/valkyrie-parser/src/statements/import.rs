use super::*;

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

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(self.path.len() + 1);
        let kind = match self.kind {
            NamespaceKind::Shared => "namespace/shared",
            NamespaceKind::Unique => "namespace/unique",
            NamespaceKind::Test => "namespace/test",
        };
        lisp += Lisp::keyword(kind);
        for id in &self.path {
            lisp += id.as_lisp();
        }
        lisp
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
        state.finish(ImportStatement { term: head, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(2);
        lisp += Lisp::keyword("using");
        lisp += match &self.term {
            ImportTermNode::Alias(v) => v.as_lisp(),
            ImportTermNode::Group(v) => v.as_lisp(),
        };
        lisp
    }
}

impl ThisParser for ImportTermNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| ImportAliasNode::parse(s).map_into())
            .or_else(|s| ImportGroupNode::parse(s).map_into())
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            ImportTermNode::Alias(v) => v.as_lisp(),
            ImportTermNode::Group(v) => v.as_lisp(),
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

    fn as_lisp(&self) -> Lisp {
        Lisp::keyword("import/alias") + self.path.as_lisp() + self.alias.as_lisp()
    }
}

impl ThisParser for ImportGroupNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, path) = parse_any_name_path(input, parse_name_id)?;
        let (state, group) = state.match_optional(parse_maybe_group)?;
        state.finish(ImportGroupNode::new(path, group.unwrap_or_default()))
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(self.group.len() + 2);
        lisp += Lisp::keyword("import/group");
        lisp += self.path.as_lisp();
        for i in self.group.iter() {
            lisp += i.as_lisp();
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
    input.begin_choice().or_else(|s| IdentifierNode::parse(s)).or_else(parse_start_as_name).end_choice()
}

fn parse_start_as_name(input: ParseState) -> ParseResult<IdentifierNode> {
    let (state, all) = input.match_str("*")?;
    state.finish(IdentifierNode::new(all, get_span(input, state)))
}

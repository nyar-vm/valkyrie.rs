use crate::{
    helpers::{ignore, parse_any_name_path, parse_eos, parse_name_join_dot},
    traits::ThisParser,
    utils::get_span,
};
use lispify::Lisp;
use std::sync::LazyLock;
use valkyrie_ast::{
    IdentifierNode, ImportAliasNode, ImportGroupNode, ImportStatementNode, ImportTermNode, NamePathNode,
    NamespaceDeclarationNode, NamespaceKind, StringLiteralNode,
};
use valkyrie_types::third_party::pex::{ParseResult, ParseState, Regex};

pub static NAMESPACE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
    namespace[*!?]?
)",
    )
    .unwrap()
});

impl ThisParser for NamespaceDeclarationNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, ns) = input.match_regex(&NAMESPACE, "NAMESPACE")?;
        let (finally, names) = state.skip(ignore).match_fn(parse)?;
        let kind = match ns.as_str() {
            "namespace*" => NamespaceKind::Test,
            _ => NamespaceKind::Unique,
        };
        finally.finish(NamespaceDeclarationNode::new(names, get_span(input, finally)).with_kind(kind))
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.path.len() + 1);
        let kind = match self.kind {
            NamespaceKind::Shared => "namespace/shared",
            NamespaceKind::Unique => "namespace/unique",
            NamespaceKind::Test => "namespace/test",
        };
        terms.push(Lisp::keyword(kind));
        terms.extend(self.path.iter().map(|id| id.as_lisp()));
        Lisp::Any(terms)
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

impl ThisParser for ImportStatementNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("using")?;
        let (state, head) = state.skip(ignore).match_fn(ImportStatementType::parse)?;
        let mut group = vec![];
        let (state, _) = state.match_fn(|s| parse_maybe_group(s, &mut group))?;
        let (finally, _) = parse_eos(state)?;
        finally.finish(ImportStatementNode { r#type: head, span: get_span(input, finally) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(2);
        terms.push(Lisp::keyword("using"));
        match &self.r#type {
            ImportStatementType::Group(v) => terms.push(v.as_lisp()),
            ImportStatementType::String(v) => terms.push(v.as_lisp()),
            ImportStatementType::Alias(v) => terms.push(v.as_lisp()),
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for ImportStatementType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| {
                let (state, names) = ImportAliasNode::parse(s)?;
                state.finish(ImportStatementType::Alias(Box::new(names)))
            })
            .or_else(|s| {
                let (state, names) = ImportGroupNode::parse(s)?;
                state.finish(ImportStatementType::Group(Box::new(names)))
            })
            .or_else(|s| {
                let (state, names) = StringLiteralNode::parse(s)?;
                state.finish(ImportStatementType::String(Box::new(names)))
            })
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ThisParser for ImportTermNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| ImportAliasNode::parse(s).map_inner(Into::into))
            .or_else(|s| ImportGroupNode::parse(s).map_inner(Into::into))
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
        let (state, path) = parse_any_name_path(input)?;
        let (state, _) = state.skip(ignore).match_str("as")?;
        let (state, alias) = state.skip(ignore).match_fn(IdentifierNode::parse)?;
        state.finish(ImportAliasNode::new(path, alias))
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Any(vec![Lisp::keyword("import/alias"), self.path.as_lisp(), self.alias.as_lisp()])
    }
}

impl ThisParser for ImportGroupNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let mut group = vec![];
        let (state, path) = parse_any_name_path(input)?;
        let (state, _) = state.match_fn(|s| parse_maybe_group(s, &mut group))?;
        state.finish(ImportGroupNode::new(path, group))
    }

    fn as_lisp(&self) -> Lisp {
        let mut items = Vec::with_capacity(self.group.len() + 2);
        items.push(Lisp::keyword("import/group"));
        items.push(self.path.as_lisp());
        items.extend(self.group.iter().map(|term| term.as_lisp()));
        Lisp::Any(items)
    }
}

/// `.? { body }`
fn parse_maybe_group<'a>(input: ParseState<'a>, items: &mut Vec<ImportTermNode>) -> ParseResult<'a, ()> {
    let (state, _) = input.skip(ignore).match_optional(parse_name_join_dot)?;
    let (state, _) = state.skip(ignore).match_str("{")?;
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

use crate::{
    helpers::{ignore, parse_any_name_path, parse_name_join_dot},
    traits::ThisParser,
};
use lispify::Lisp;
use std::sync::LazyLock;
use valkyrie_ast::{
    IdentifierNode, ImportAliasNode, ImportGroupNode, ImportStatementNode, ImportTermNode, NamespaceDeclarationNode,
    NamespaceKind,
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
        finally.finish(NamespaceDeclarationNode::new(names, &finally.away_from(input)).with_kind(kind))
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
    let (state, _) = input
        .begin_choice()
        .or_else(|s| s.match_char('.').map_inner(|_| ()))
        .or_else(|s| s.match_str("::").map_inner(|_| ()))
        .or_else(|s| s.match_char('∷').map_inner(|_| ()))
        .end_choice()?;
    let (state, id) = state.match_fn(|s| IdentifierNode::parse(s))?;
    names.push(id);
    state.finish(())
}

impl ThisParser for ImportStatementNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
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
        /// `.? { body }`
        fn parse_maybe_group(input: ParseState) -> ParseResult<Vec<ImportTermNode>> {
            let (state, _) = input.match_optional(parse_name_join_dot)?;
            todo!()
        }

        let (state, path) = parse_any_name_path(input)?;
        let (state, group) = state.skip(ignore).match_fn(parse_maybe_group)?;
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

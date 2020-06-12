use crate::{helpers::ignore, traits::ThisParser};
use lispify::Lisp;
use std::sync::LazyLock;
use valkyrie_ast::{IdentifierNode, NamespaceDeclareNode, NamespaceKind};
use valkyrie_types::third_party::pex::{ParseResult, ParseState, Regex};

pub static NAMESPACE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
    namespace[*!?]?
)",
    )
    .unwrap()
});

impl ThisParser for NamespaceDeclareNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, ns) = input.match_regex(&NAMESPACE, "NAMESPACE")?;
        let (finally, names) = state.skip(ignore).match_fn(parse)?;
        let kind = match ns.as_str() {
            "namespace*" => NamespaceKind::Test,
            _ => NamespaceKind::Unique,
        };
        finally.finish(NamespaceDeclareNode::new(names, &finally.away_from(input)).with_kind(kind))
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

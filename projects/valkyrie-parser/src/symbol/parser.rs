use super::*;
use crate::traits::ThisParser;
use lispify::{Lisp, LispSymbol};
use valkyrie_ast::IdentifierNode;

pub static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
      [∞∅]
    | (?P<regular>(?:\p{XID_Start}|_)\p{XID_Continue}*)
    | `(?P<escaped>(?:\\.|[^`])*)`
)",
    )
    .unwrap()
});

impl ThisParser for IdentifierNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&IDENTIFIER, "IDENTIFIER")?;
        let id = IdentifierNode::new(m.as_str(), m.range());
        state.finish(id)
    }

    fn as_lisp(&self) -> Lisp {
        LispSymbol { name: self.name.clone(), path: vec![] }.into()
    }
}

impl ThisParser for NamepathNode {
    /// `id (~ :: ~ b)*`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let mut names = Vec::new();
        let (state, id) = input.match_fn(IdentifierNode::parse)?;
        names.push(id);
        let (state, _) = state.match_repeats(|s| pare_colon_id(s, &mut names))?;
        state.finish(NamepathNode::new(names, state.away_from(input)))
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = self.names.iter().map(|s| s.name.clone());
        let first = terms.next().unwrap_or_default();
        LispSymbol { name: first, path: terms.collect() }.into()
    }
}

fn pare_colon_id<'i>(input: ParseState<'i>, names: &mut Vec<IdentifierNode>) -> ParseResult<'i, ()> {
    let (state, _) = input
        .begin_choice()
        .or_else(|s| s.match_str("::").map_inner(|_| ()))
        .or_else(|s| s.match_char('∷').map_inner(|_| ()))
        .end_choice()?;
    let (state, id) = state.match_fn(|s| IdentifierNode::parse(s))?;
    names.push(id);
    state.finish(())
}

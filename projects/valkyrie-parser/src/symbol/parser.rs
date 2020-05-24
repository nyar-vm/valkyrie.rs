use super::*;
use crate::traits::ThisParser;
use lispify::{Lisp, LispSymbol};
use valkyrie_ast::ValkyrieIdentifier;

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

impl ThisParser for ValkyrieIdentifier {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&IDENTIFIER, "IDENTIFIER")?;
        let id = ValkyrieIdentifier::new(m.as_str(), &m.range());
        state.finish(id)
    }

    fn parse_many(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        LispSymbol { name: self.name.clone(), path: vec![] }.into()
    }
}

impl ValkyrieNamepath {
    /// `id (~ :: ~ b)*`
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let mut names = Vec::new();
        let (state, id) = input.match_fn(ValkyrieIdentifier::parse)?;
        names.push(id);
        let (state, _) = state.match_repeats(|s| pare_colon_id(s, &mut names))?;
        state.finish(ValkyrieNamepath { names, range: state.away_from(input) })
    }
}

fn pare_colon_id<'i>(input: ParseState<'i>, names: &mut Vec<ValkyrieIdentifier>) -> ParseResult<'i, ()> {
    let (state, _) = input
        .begin_choice()
        .or_else(|s| s.match_str("::").map_inner(|_| ()))
        .or_else(|s| s.match_char('∷').map_inner(|_| ()))
        .end_choice()?;
    let (state, id) = state.match_fn(|s| ValkyrieIdentifier::parse(s))?;
    names.push(id);
    state.finish(())
}

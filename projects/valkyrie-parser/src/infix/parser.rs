use super::*;
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};
use regex::Regex;
use std::{str::FromStr, sync::LazyLock};

static INFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      [.]{2}[=<]
    # | [.]{1,3}
    # | [⟦⟧⁅⁆⟬⟭{}\[\]()]
    | [$§¶^]
    | @[*!?@]?
    | (\b(not)\b\s+)?\b(in)\b
    # start with <, >
    | >{1,3} | >= | /> | ≥ | ⩾ | ≫ | ⋙
    | <{1,3} | <= | </ | ≤ | ⩽ | ≪ | ⋘ |  <: | <! 
    # start with :
    | :> | := | ≔
    # start with -
    | -= | -> | ⟶ | -{1,2}
    # start with ~
    | ~> | ~
    # start with +
    | [+]= | [+]> | [+]{1,2}
    # start with *
    | [*]=?
    # start with / or % or ÷
    | /=?
    | ÷=?
    | %=?
    # start with &
    | &> | &{1,2} | ≻
    | [|]> | [|]{1,2} | ⊁
    | ⊻=? | ⊼=? | ⊽=? | [⩕⩖]
    # start with !
    | != | ≠ | !
    # start with ?
    | [?]{3} | [?]
    # start with =
    | => | ⇒
    | === | == | =
    # unicode
    | [∈∊∉⊑⋢⨳∀∁∂∃∄¬±√∛∜⊹⋗]
    | [↻↺⇆↹⇄⇋⇌⇅]
)"#,
    )
    .unwrap()
});

impl FromStr for ValkyrieInfix {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieInfix::parse)
    }
}

impl ValkyrieInfix {
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&INFIX, "INFIX")?;
        let id = ValkyrieInfix::new(m.as_str(), state.away_from(input));
        state.finish(id)
    }
}

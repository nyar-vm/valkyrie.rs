use super::*;
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};
use regex::Regex;
use std::{str::FromStr, sync::LazyLock};

static PREFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)
      \#
    | [.]{2}[=<]
    | [.]{1,3}
    | [{}\[\]()]
    | [,;$§¶^]
    | @[*!?@]?
    | [!]?
    # start with <, >
    | >{1,3} | >= | /> | ≥ | ⩾ | ≫
    | <{1,3} | <= | </ | ≤ | ⩽ | <: | <! 
    # start with :
    | ∷ | :: | :> | := | ≔ | :
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
    | [⟦⟧⁅⁆⟬⟭]
    | [↻↺⇆↹⇄⇋⇌⇅]
"#,
    )
    .unwrap()
});

impl FromStr for ValkyriePostfix {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyriePostfix::parse)
    }
}

impl ValkyriePostfix {
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&Prefix, "INFIX")?;
        let mut normalized = String::with_capacity(m.len());
        for c in m.as_str().chars() {
            match c {
                ' ' => continue,
                '∈' | '∊' => normalized.push_str("in"),
                '∉' => normalized.push_str("!in"),
                _ => normalized.push(c),
            }
        }
        let id = ValkyriePostfix { normalized: m.as_str().to_string(), range: state.away_from(input) };
        state.finish(id)
    }
}

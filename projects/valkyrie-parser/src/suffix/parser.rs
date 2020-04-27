use super::*;
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};
use regex::Regex;
use std::{str::FromStr, sync::LazyLock};

static POSTFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      [!?]
    # Temperature
    | [℃℉]
    # Percents
    | [%‰‱]
    # Transpose,adjoint, conjugate
    | [ᵀᴴ]
)"#,
    )
    .unwrap()
});

impl FromStr for ValkyrieSuffix {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieSuffix::parse)
    }
}

impl ValkyrieSuffix {
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&POSTFIX, "POSTFIX")?;
        let mut normalized = String::with_capacity(m.len());
        for c in m.as_str().chars() {
            match c {
                // ' ' => continue,
                // '∈' | '∊' => normalized.push_str("in"),
                // '∉' => normalized.push_str("!in"),
                _ => normalized.push(c),
            }
        }
        let id = ValkyrieSuffix { normalized: m.as_str().to_string(), range: state.away_from(input) };
        state.finish(id)
    }
}

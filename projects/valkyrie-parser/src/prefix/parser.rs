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
      [¬!~]
    | [±√∛∜]
"#,
    )
    .unwrap()
});

impl FromStr for ValkyriePrefix {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyriePrefix::parse)
    }
}

impl ValkyriePrefix {
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&PREFIX, "PREFIX")?;
        let mut normalized = String::with_capacity(m.len());
        for c in m.as_str().chars() {
            match c {
                // ' ' => continue,
                '¬' => normalized.push_str("!"),
                _ => normalized.push(c),
            }
        }
        let id = ValkyriePrefix { normalized, range: state.away_from(input) };
        state.finish(id)
    }
}
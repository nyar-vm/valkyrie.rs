use std::str::FromStr;
use std::sync::LazyLock;
use pex::{ParseResult, ParseState, StopBecause};
use pex::helpers::{make_from_str, whitespace};
use regex::Regex;
use super::*;


impl FromStr for ValkyrieNumber {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

// 16^^AEF
pub static NUMBER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?ux)
    [1-9](_*[0-9])*
|   [0-9]
    ").unwrap()
});

impl ValkyrieNumber {
    /// - regular: `\p{XID_Start}|_)\p{XID_Continue}*`
    /// - escaped: ``` `(\.|[^`])*` ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&NUMBER, "NUMBER")?;
        let mut value = String::with_capacity(m.as_str().len());
        for c in m.as_str().chars() {
            if c != '_' {
                value.push(c);
            }
        }
        let (state, unit) = state.match_optional(ValkyrieNumber::parse_unit)?;
        let id = ValkyrieNumber {
            value,
            unit,
            range: state.away_from(input),
        };
        state.finish(id)
    }
    fn parse_unit(input: ParseState) -> ParseResult<ValkyrieIdentifier> {
        let (state, _) = input.match_optional(|s| s.match_char('_'))?;
        let (state, id) = state.match_fn(ValkyrieIdentifier::parse)?;
        state.finish(id)
    }
}

#[test]
pub fn test() {
    let input = "1__2__3__4__cm";
    let n = ValkyrieNumber::from_str(input).unwrap();
    println!("{:#?}", n)
}

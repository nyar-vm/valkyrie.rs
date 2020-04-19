use std::str::FromStr;
use std::sync::LazyLock;
use pex::{ParseResult, ParseState, StopBecause, ZeroBytePattern};
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
        let (state, unit) = state.match_optional(parse_unit)?;
        let mut value = String::with_capacity(m.as_str().len());
        for c in m.as_str().chars() {
            if c != '_' {
                value.push(c);
            }
        }
        let id = ValkyrieNumber {
            value,
            unit,
            range: state.away_from(input),
        };
        state.finish(id)
    }
}

// 16^^AEF
pub static BYTES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?ux)
    ⍚[0-9a-fA-F](_*[0-9a-fA-F])* # hex
|   ⍙[0-7](_*[0-7])*             # octal
|   ⍜[01](_*[01])*               # binary
    ").unwrap()
});

// ZeroBytePattern::new(&[("⍚", 16), ("⍙", 8), ("⍜", 2)]);
impl ValkyrieBytes {
    /// ```js
    /// ⍚F => [15]
    /// ⍚FF => [255]
    /// ⍚FFF => [15, 255]
    /// ```
    fn parse_hex(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&NUMBER, "NUMBER")?;

    }
}

fn parse_unit(input: ParseState) -> ParseResult<ValkyrieIdentifier> {
    let (state, _) = input.match_optional(|s| s.match_char('_'))?;
    let (state, id) = state.match_fn(ValkyrieIdentifier::parse)?;
    state.finish(id)
}

fn parse_base(input: &str, base: u32) -> usize {
    let mut offset = 0;
    for c in input.chars() {
        match c {
            '_' => offset += 1,
            c if c.is_digit(base) => offset += 1,
            _ => break,
        }
    }
    offset
}

#[test]
pub fn test() {
    let n = ValkyrieNumber::from_str("1234cm").unwrap();
    let n = ValkyrieNumber::from_str("1_2_3_4_cm").unwrap();
    let n = ValkyrieNumber::from_str("1__2__3__4__cm").unwrap();
    println!("{:#?}", n)
}

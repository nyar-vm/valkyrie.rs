use std::str::{Chars, FromStr};
use std::sync::LazyLock;
use bit_set::BitSet;
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

impl FromStr for ValkyrieBytes {
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


pub static BYTES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?ux)
    ⍚(_*[0-9a-fA-F])* # hex
|   ⍙(_*[0-7])*       # octal
|   ⍜(_*[01])*        # binary
    ").unwrap()
});

// ZeroBytePattern::new(&[("⍚", 16), ("⍙", 8), ("⍜", 2)]);
impl ValkyrieBytes {
    /// ```js
    /// ⍚F => [15]
    /// ⍚FF => [255]
    /// ⍚FFF => [15, 255]
    /// ⍚F_F_F_F => [255, 255]
    /// ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, s) = input.match_regex(&BYTES, "BYTES").map_inner(|m| m.as_str())?;
        let (state, unit) = state.match_optional(parse_unit)?;
        let mut chars = s.chars();
        let value = match chars.next() {
            Some('⍜') => {
                let out = parse_bin(s);
                println!("out: {:?}", out);
                out
            }
            Some('⍙') => { todo!() }
            Some('⍚') => { todo!() }
            _ => StopBecause::missing_character('⍚', state.start_offset)?,
        };
        state.finish(ValkyrieBytes {
            bits: value,
            unit,
            range: Default::default(),
        })
    }
}

fn parse_bin(raw: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(raw.len() / 8);
    let mut byte = 0;
    let mut index: u8 = 0;
    for char in raw.chars().rev() {
        match char {
            '0' => {
                index += 1;
            }
            '1' => {
                byte |= 1 << index;
                index += 1;
            }
            '_' => {}
            _ => {
                break;
            }
        }
        if index == 8 {
            bytes.push(byte);
            byte = 0;
            index = 0;
        }
    }
    bytes
}

fn parse_unit(input: ParseState) -> ParseResult<ValkyrieIdentifier> {
    let (state, _) = input.match_optional(|s| s.match_char('_'))?;
    let (state, id) = state.match_fn(ValkyrieIdentifier::parse)?;
    state.finish(id)
}

fn count_base(input: &str, base: u32) -> usize {
    let mut count = 0;
    for c in input.chars() {
        match c {
            c if c.is_digit(base) => count += 1,
            _ => continue,
        }
    }
    count
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
pub fn test_hex() {
    let n = ValkyrieBytes::from_str("⍜0_0000_0101").unwrap();
    println!("{:#?}", n.bits)
}


#[test]
pub fn test() {
    let n = ValkyrieNumber::from_str("1234cm").unwrap();
    let n = ValkyrieNumber::from_str("1_2_3_4_cm").unwrap();
    let n = ValkyrieNumber::from_str("1__2__3__4__cm").unwrap();
    println!("{:#?}", n)
}

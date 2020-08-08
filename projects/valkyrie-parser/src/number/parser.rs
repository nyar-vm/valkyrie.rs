use super::*;
use crate::{traits::ThisParser, utils::get_span};
use lispify::Lisp;
use valkyrie_ast::NumberLiteralNode;

impl FromStr for ValkyrieBytes {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

// 16^^AEF
pub static NUMBER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
    [0-9](_*[0-9])*([.][0-9](_*[0-9])*)?
)",
    )
    .unwrap()
});

impl ThisParser for NumberLiteralNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&NUMBER, "NUMBER")?;
        let (state, unit) = state.match_optional(parse_unit)?;
        let mut value = String::with_capacity(m.as_str().len());
        for c in m.as_str().chars() {
            if c != '_' {
                value.push(c);
            }
        }
        let mut number = NumberLiteralNode::new(m.as_str(), get_span(input, state));
        number.unit = unit;
        state.finish(number)
    }

    fn as_lisp(&self) -> Lisp {
        LispNumber { number: self.value.clone(), unit: self.unit.clone().map(|s| s.name).unwrap_or_default() }.into()
    }
}

pub static BYTES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ux)
    ⍚(_*[0-9A-F])* # hex
|   ⍙(_*[0-7])*       # octal
|   ⍜(_*[01])*        # binary
    ",
    )
    .unwrap()
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
        let mut value = match chars.next() {
            Some('⍜') => parse_bin(s),
            Some('⍙') => parse_hex(s),
            Some('⍚') => parse_hex(s),
            _ => StopBecause::missing_character('⍚', state.start_offset)?,
        };
        match chars.next() {
            Some(_) => {
                value.reverse();
                value.shrink_to_fit();
            }
            None => value = vec![],
        }
        state.finish(ValkyrieBytes { bytes: value, unit, range: Default::default() })
    }
}

fn parse_bin(raw: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(raw.len() / 8);
    let mut byte = 0;
    let mut index: u8 = 0;
    for char in raw.chars().rev() {
        match char {
            '0' => {}
            '1' => byte |= 1 << index,
            _ => {
                continue;
            }
        }
        index += 1;
        if index == 8 {
            bytes.push(byte);
            byte = 0;
            index = 0;
        }
    }
    bytes.push(byte);
    bytes
}

fn parse_hex(raw: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(raw.len() / 2);
    let mut byte = 0;
    let mut index: u8 = 0;
    for char in raw.chars().rev() {
        match char {
            '0'..='9' => byte |= (char as u8 - b'0') << index,
            'A'..='F' => byte |= (char as u8 - b'A' + 10) << index,
            _ => continue,
        }
        index += 4;
        if index == 8 {
            bytes.push(byte);
            byte = 0;
            index = 0;
        }
    }
    bytes.push(byte);
    bytes
}

fn parse_unit(input: ParseState) -> ParseResult<IdentifierNode> {
    let (state, _) = input.match_optional(|s| s.match_char('_'))?;
    let (state, id) = state.match_fn(IdentifierNode::parse)?;
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

use super::*;

/// A number literal.
#[derive(Debug, Clone, Eq, Hash)]
pub struct StringTemplateNode {
    /// The raw string of the number.
    pub bytes: Vec<u8>,
    /// The unit of the number, if any.
    pub unit: Option<IdentifierNode>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl PartialEq for StringTemplateNode {
    fn eq(&self, other: &Self) -> bool {
        self.bytes.eq(&other.bytes) && self.unit.eq(&other.unit)
    }
}

impl FromStr for StringTemplateNode {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

impl ThisParser for StringLiteralNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, unit) = input.match_optional(IdentifierNode::parse)?;
        let (state, pair) = state
            .begin_choice()
            .or_else(|s| quotation_pair_nested(s, '\''))
            .or_else(|s| quotation_pair_nested(s, '"'))
            .or_else(|s| quotation_pair(s, '«', '»'))
            .end_choice()?;

        state.finish(StringLiteralNode { value: pair.body.as_string(), unit, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        ListString { text: self.value.to_owned(), unit: self.unit.clone().map(|u| u.name).unwrap_or_default() }.into()
    }
}

// ZeroBytePattern::new(&[("⍚", 16), ("⍙", 8), ("⍜", 2)]);
impl StringTemplateNode {
    /// ```js
    /// ⍚F => [15]
    /// ⍚FF => [255]
    /// ⍚FFF => [15, 255]
    /// ⍚F_F_F_F => [255, 255]
    /// ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }
}

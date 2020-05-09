use super::*;
use crate::expression::ValkyrieExpression;
use pex::{
    helpers::{make_from_str, quotation_pair, quotation_pair_nested, whitespace},
    ParseResult, ParseState, StopBecause,
};

use std::str::FromStr;

impl FromStr for ValkyrieString {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);

        make_from_str(state, ValkyrieString::parse)
    }
}

impl FromStr for ValkyrieTemplate {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

impl ValkyrieString {
    /// - regular: `\p{XID_Start}|_)\p{XID_Continue}*`
    /// - escaped: ``` `(\.|[^`])*` ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, unit) = input.match_optional(ValkyrieIdentifier::parse)?;
        let (state, pair) = state
            .begin_choice()
            .or_else(|s| quotation_pair_nested(s, '\''))
            .or_else(|s| quotation_pair_nested(s, '"'))
            .or_else(|s| quotation_pair(s, '«', '»'))
            .end_choice()?;

        state.finish(ValkyrieString { value: pair.body.as_string(), unit, range: state.away_from(input) })
    }
}

impl From<ValkyrieString> for ValkyrieExpression {
    fn from(value: ValkyrieString) -> Self {
        ValkyrieExpression::String(Box::new(value))
    }
}

// ZeroBytePattern::new(&[("⍚", 16), ("⍙", 8), ("⍜", 2)]);
impl ValkyrieTemplate {
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

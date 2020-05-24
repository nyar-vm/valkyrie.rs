use super::*;
use valkyrie_ast::ValkyrieIdentifier;

impl FromStr for ValkyrieDotCall {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieDotCall::parse)
    }
}

// ZeroBytePattern::new(&[("⍚", 16), ("⍙", 8), ("⍜", 2)]);
impl ValkyrieDotCall {
    /// ```js
    /// .id(xxx)
    /// ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('.')?;
        let (state, caller) = state.skip(ignore).match_fn(ValkyrieIdentifier::parse)?;
        let (finally, args) = state.skip(ignore).match_fn(ValkyrieApply::parse)?;
        finally.finish(ValkyrieDotCall {
            base: ValkyrieExpression::Placeholder,
            caller,
            terms: args.terms,
            range: finally.away_from(input),
        })
    }
}

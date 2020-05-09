use super::*;

impl FromStr for ValkyrieTableTerm {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieTableTerm::parse)
    }
}

impl FromStr for ValkyrieApply {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieApply::parse)
    }
}

impl FromStr for ValkyriePair {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyriePair::parse)
    }
}

// ZeroBytePattern::new(&[("⍚", 16), ("⍙", 8), ("⍜", 2)]);
impl ValkyrieApply {
    /// ```js
    /// []
    /// [term (, term)* ,?]
    /// ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")");
        let (state, terms) = pat.consume(input, ignore, ValkyrieTableTerm::parse)?;
        state.finish(ValkyrieApply { base: ValkyrieExpression::Placeholder, terms: terms.body, range: state.away_from(input) })
    }
}

impl ValkyrieTableTerm {
    /// - `start? ~ : ~ end? (~ : ~ step?)? `
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(ValkyrieTableTerm::parse_pair).or_else(ValkyrieTableTerm::parse_term).end_choice()
    }
    /// - `key ~ : ~ value`
    pub fn parse_pair(input: ParseState) -> ParseResult<ValkyrieTableTerm> {
        let (state, term) = input.skip(ignore).match_fn(ValkyriePair::parse)?;
        state.finish(ValkyrieTableTerm::Pair(term))
    }
    /// - `term`
    pub fn parse_term(input: ParseState) -> ParseResult<ValkyrieTableTerm> {
        let (state, term) = input.skip(ignore).match_fn(ValkyrieExpression::parse)?;
        state.finish(ValkyrieTableTerm::Item(term))
    }
}

impl ValkyriePair {
    /// - `key ~ : ~ value`
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, key) = input.match_fn(ValkyriePair::parse_key)?;
        let (state, _) = state.skip(ignore).match_char(':')?;
        let (state, value) = state.skip(ignore).match_fn(ValkyrieExpression::parse)?;
        state.finish(ValkyriePair { key, value })
    }
    #[inline(always)]
    fn parse_key(input: ParseState) -> ParseResult<ValkyrieIdentifier> {
        ValkyrieIdentifier::parse(input)
    }
    #[inline(always)]
    pub fn parse_value(input: ParseState) -> ParseResult<ValkyrieExpression> {
        ValkyrieExpression::parse(input)
    }
}

/// `~ : ~ step?`
fn maybe_step(input: ParseState) -> ParseResult<Option<ValkyrieExpression>> {
    let (state, _) = input.skip(ignore).match_char(':')?;
    let (state, term) = state.skip(ignore).match_optional(ValkyrieExpression::parse)?;
    state.finish(term)
}

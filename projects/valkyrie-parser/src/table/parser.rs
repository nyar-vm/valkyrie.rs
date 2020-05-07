use super::*;

impl FromStr for ValkyrieViewTerm {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieViewTerm::parse)
    }
}

impl FromStr for ValkyrieView {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieView::parse)
    }
}

// ZeroBytePattern::new(&[("⍚", 16), ("⍙", 8), ("⍜", 2)]);
impl ValkyrieView {
    /// ```js
    /// []
    /// [term (, term)* ,?]
    /// ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, ValkyrieViewTerm::parse)?;
        state.finish(ValkyrieView { base: ValkyrieExpression::Placeholder, terms: terms.body, range: state.away_from(input) })
    }
}

impl ValkyrieViewTerm {
    /// - `start? ~ : ~ end? (~ : ~ step?)? `
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(ValkyrieViewTerm::parse_ranged).or_else(ValkyrieViewTerm::parse_single).end_choice()
    }
    /// - `start? ~ : ~ end? (~ : ~ step?)? `
    pub fn parse_ranged(input: ParseState) -> ParseResult<ValkyrieViewTerm> {
        let (state, start) = input.match_optional(ValkyrieExpression::parse)?;
        let (state, _) = state.skip(ignore).match_char(':')?;
        let (state, end) = state.skip(ignore).match_optional(ValkyrieExpression::parse)?;
        let (state, step) = state.match_optional(maybe_step)?;
        state.finish(ValkyrieViewTerm::Range { start, end, step: step.flatten(), range: state.away_from(input) })
    }
    /// - `term`
    pub fn parse_single(input: ParseState) -> ParseResult<ValkyrieViewTerm> {
        let (state, term) = input.skip(ignore).match_fn(ValkyrieExpression::parse)?;
        state.finish(ValkyrieViewTerm::Index { element: term, range: state.away_from(input) })
    }
}

/// `~ : ~ step?`
fn maybe_step(input: ParseState) -> ParseResult<Option<ValkyrieExpression>> {
    let (state, _) = input.skip(ignore).match_char(':')?;
    let (state, term) = state.skip(ignore).match_optional(ValkyrieExpression::parse)?;
    state.finish(term)
}

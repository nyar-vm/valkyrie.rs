use super::*;

impl FromStr for ValkyrieSliceTerm {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieSliceTerm::parse)
    }
}

impl FromStr for ValkyrieSlice {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieSlice::parse)
    }
}

// ZeroBytePattern::new(&[("⍚", 16), ("⍙", 8), ("⍜", 2)]);
impl ValkyrieSlice {
    /// ```js
    /// []
    /// [term (, term)* ,?]
    /// ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, ValkyrieSliceTerm::parse)?;
        state.finish(ValkyrieSlice { base: ValkyrieExpression::Placeholder, terms: terms.body, range: state.away_from(input) })
    }
}

impl ValkyrieSliceTerm {
    /// - `start? ~ : ~ end? (~ : ~ step?)? `
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(maybe_slice).or_else(maybe_index).end_choice()
    }
}

fn maybe_slice(input: ParseState) -> ParseResult<ValkyrieSliceTerm> {
    let (state, start) = input.match_optional(ValkyrieExpression::parse)?;
    let (state, _) = state.skip(ignore).match_char('~')?;
    let (state, end) = state.skip(ignore).match_optional(ValkyrieExpression::parse)?;
    let (state, step) = state.match_optional(maybe_step)?;
    state.finish(ValkyrieSliceTerm { is_index: false, start, end, step, range: state.away_from(input) })
}

fn maybe_step(input: ParseState) -> ParseResult<ValkyrieExpression> {
    let (state, _) = input.skip(ignore).match_char(':')?;
    let (state, term) = state.skip(ignore).match_fn(ValkyrieExpression::parse)?;
    state.finish(term)
}

fn maybe_index(input: ParseState) -> ParseResult<ValkyrieSliceTerm> {
    let (state, term) = input.skip(ignore).match_fn(ValkyrieExpression::parse)?;
    state.finish(ValkyrieSliceTerm { is_index: true, start: Some(term), end: None, step: None, range: state.away_from(input) })
}

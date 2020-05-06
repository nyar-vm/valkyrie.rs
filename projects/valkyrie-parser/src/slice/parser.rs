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
        todo!()
    }
}

fn maybe_step<T>(input: ParseState) -> ParseResult<ValkyrieExpression> {
    let (state, _) = input.skip(ignore).match_char(':')?;
}

fn maybe_index<T>(input: ParseState) -> ParseResult<Vec<T>> {
    input.skip(ignore).match_fn()
}

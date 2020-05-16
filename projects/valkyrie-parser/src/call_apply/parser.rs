use super::*;

impl FromStr for ValkyrieApply {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieApply::parse)
    }
}

impl ValkyrieApply {
    /// `(` ~ `)` | `(` [term](ValkyrieTableTerm::parse) ( ~ `,` ~ [term](ValkyrieTableTerm::parse))* `,`? `)`
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")");
        let (state, terms) = pat.consume(input, ignore, ValkyrieTableTerm::parse)?;
        state.finish(ValkyrieApply { base: ValkyrieExpression::Placeholder, terms: terms.body, range: state.away_from(input) })
    }
}

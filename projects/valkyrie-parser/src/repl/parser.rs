use super::*;

impl FromStr for ValkyrieViewTerm {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieViewTerm::parse)
    }
}

impl FromStr for ValkyrieREPL {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieREPL::parse)
    }
}

impl ValkyrieREPL {
    /// `[` ~ `]` | `[` [term](ValkyrieViewTerm::parse) ( ~ `,` ~ [term](ValkyrieViewTerm::parse))* `,`? `]`
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, ValkyrieViewTerm::parse)?;
        state.finish(ValkyrieREPL { base: ValkyrieExpression::Placeholder, terms: terms.body, range: state.away_from(input) })
    }
}

impl ValkyrieViewTerm {
    /// [range](ValkyrieViewTerm::parse_ranged) | [index](ValkyrieViewTerm::parse_single)
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(ValkyrieViewTerm::parse_ranged).or_else(ValkyrieViewTerm::parse_single).end_choice()
    }
    /// [start](ValkyrieExpression::parse)? ~ `:` ~ [end](ValkyrieExpression::parse)? (~ `:` ~ [step](ValkyrieExpression::parse)?)?
    pub fn parse_ranged(input: ParseState) -> ParseResult<ValkyrieViewTerm> {
        let (state, start) = input.match_optional(ValkyrieExpression::parse)?;
        let (state, _) = state.skip(ignore).match_char(':')?;
        let (state, end) = state.skip(ignore).match_optional(ValkyrieExpression::parse)?;
        let (state, step) = state.match_optional(maybe_step)?;
        state.finish(ValkyrieViewTerm::Range { start, end, step: step.flatten(), range: state.away_from(input) })
    }
    /// - [term](ValkyrieExpression::parse)
    pub fn parse_single(input: ParseState) -> ParseResult<ValkyrieViewTerm> {
        let (state, term) = ValkyrieExpression::parse(input)?;
        state.finish(ValkyrieViewTerm::Index { element: term, range: state.away_from(input) })
    }
}

/// `~ : ~ step?`
fn maybe_step(input: ParseState) -> ParseResult<Option<ValkyrieExpression>> {
    let (state, _) = input.skip(ignore).match_char(':')?;
    let (state, term) = state.skip(ignore).match_optional(ValkyrieExpression::parse)?;
    state.finish(term)
}

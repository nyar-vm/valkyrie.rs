use super::*;

impl FromStr for ViewTermNode {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ViewTermNode::parse)
    }
}

impl FromStr for ViewNode {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ViewNode::parse)
    }
}

impl ViewNode {
    /// `[` ~ `]` | `[` [term](ViewTermNode::parse) ( ~ `,` ~ [term](ViewTermNode::parse))* `,`? `]`
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, ViewTermNode::parse)?;
        state.finish(ViewNode { base: ValkyrieExpression::Placeholder, terms: terms.body, range: state.away_from(input) })
    }
}

impl ViewTermNode {
    /// [range](ViewTermNode::parse_ranged) | [index](ViewTermNode::parse_single)
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(ViewTermNode::parse_ranged).or_else(ViewTermNode::parse_single).end_choice()
    }
    /// [start](ValkyrieExpression::parse)? ~ `:` ~ [end](ValkyrieExpression::parse)? (~ `:` ~ [step](ValkyrieExpression::parse)?)?
    pub fn parse_ranged(input: ParseState) -> ParseResult<ViewTermNode> {
        let (state, start) = input.match_optional(ValkyrieExpression::parse)?;
        let (state, _) = state.skip(ignore).match_char(':')?;
        let (state, end) = state.skip(ignore).match_optional(ValkyrieExpression::parse)?;
        let (state, step) = state.match_optional(maybe_step)?;
        state.finish(ViewTermNode::Range { start, end, step: step.flatten(), range: state.away_from(input) })
    }
    /// - [term](ValkyrieExpression::parse)
    pub fn parse_single(input: ParseState) -> ParseResult<ViewTermNode> {
        let (state, term) = ValkyrieExpression::parse(input)?;
        state.finish(ViewTermNode::Index { element: term, range: state.away_from(input) })
    }
}

/// `~ : ~ step?`
fn maybe_step(input: ParseState) -> ParseResult<Option<ValkyrieExpression>> {
    let (state, _) = input.skip(ignore).match_char(':')?;
    let (state, term) = state.skip(ignore).match_optional(ValkyrieExpression::parse)?;
    state.finish(term)
}

use super::*;
use crate::{apply::ValkyriePair, expression::ValkyrieExpression};

impl FromStr for ValkyrieTable {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieTable::parse)
    }
}

impl ValkyrieTable {
    /// `[` ~ `]` | `[` [term](ValkyrieTableTerm::parse) ( ~ `,` ~ [term](ValkyrieTableTerm::parse))* `,`? `]`
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, ValkyrieTableTerm::parse)?;
        state.finish(ValkyrieTable { terms: terms.body, range: state.away_from(input) })
    }
}

impl ValkyrieTableTerm {
    /// - [start]()? ~ `:` ~ [end]()? (~ `:` ~ [step]?)?
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(Self::parse_pair).or_else(Self::parse_term).end_choice()
    }
    /// - `key ~ : ~ value`
    pub fn parse_pair(input: ParseState) -> ParseResult<ValkyrieTableTerm> {
        let (state, term) = ValkyriePair::parse(input)?;
        state.finish(ValkyrieTableTerm::Pair(term))
    }
    /// - `term`
    pub fn parse_term(input: ParseState) -> ParseResult<ValkyrieTableTerm> {
        let (state, term) = ValkyrieExpression::parse(input)?;
        state.finish(ValkyrieTableTerm::Item(term))
    }
}

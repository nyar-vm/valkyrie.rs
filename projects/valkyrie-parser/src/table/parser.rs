use super::*;
use crate::traits::ThisParser;
use valkyrie_ast::IdentifierNode;

impl FromStr for TableNode {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, TableNode::parse)
    }
}

impl ThisParser for TableNode {
    /// `[` ~ `]` | `[` [term](TableTermNode::parse) ( ~ `,` ~ [term](TableTermNode::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, TableTermNode::parse)?;
        state.finish(TableNode { index0: true, terms: terms.body, range: state.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::function("table"));
        for term in &self.terms {
            terms.push(term.lispify().into());
        }
        Lisp::Any(terms)
    }
}

impl FromStr for TableTermNode {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, TableTermNode::parse)
    }
}
impl FromStr for PairNode {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, PairNode::parse)
    }
}

impl TableTermNode {
    /// - [start]()? ~ `:` ~ [end]()? (~ `:` ~ [step]?)?
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(Self::parse_pair).or_else(Self::parse_term).end_choice()
    }
    /// - `key ~ : ~ value`
    pub fn parse_pair(input: ParseState) -> ParseResult<TableTermNode> {
        let (state, term) = PairNode::parse(input)?;
        state.finish(TableTermNode::Pair(term))
    }
    /// - `term`
    pub fn parse_term(input: ParseState) -> ParseResult<TableTermNode> {
        let (state, term) = ValkyrieExpression::parse(input)?;
        state.finish(TableTermNode::Item(term))
    }
}

impl PairNode {
    /// [key](PairNode::parse_key) ~ `:` ~ [value](PairNode::parse_value)
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, key) = input.match_fn(Self::parse_key)?;
        let (state, _) = state.skip(ignore).match_char(':')?;
        let (state, value) = state.skip(ignore).match_fn(Self::parse_value)?;
        state.finish(PairNode { key, value })
    }
    /// [key](IdentifierNode::parse)
    #[inline(always)]
    fn parse_key(input: ParseState) -> ParseResult<IdentifierNode> {
        IdentifierNode::parse(input)
    }
    /// [value](ValkyrieExpression::parse)
    #[inline(always)]
    pub fn parse_value(input: ParseState) -> ParseResult<ValkyrieExpression> {
        ValkyrieExpression::parse(input)
    }
}

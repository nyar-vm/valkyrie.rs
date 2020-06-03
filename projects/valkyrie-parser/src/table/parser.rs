use super::*;
use crate::traits::ThisParser;
use valkyrie_ast::{IdentifierNode, PairNode, TableKind, TableNode, TableTermNode};

impl ThisParser for TableNode<ValkyrieExpression> {
    /// `[` ~ `]` | `[` [term](TableTermNode::parse) ( ~ `,` ~ [term](TableTermNode::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, TableTermNode::parse)?;
        state.finish(TableNode { kind: TableKind::OffsetTable, terms: terms.body, range: state.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::function("table"));
        for term in &self.terms {
            terms.push(term.as_lisp().into());
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for TableTermNode<ValkyrieExpression> {
    /// - [start]()? ~ `:` ~ [end]()? (~ `:` ~ [step]?)?
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(parse_pair).or_else(parse_term).end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            TableTermNode::Item(v) => v.lispify(),
            TableTermNode::Pair(v) => v.as_lisp(),
        }
    }
}

impl ThisParser for PairNode<IdentifierNode, ValkyrieExpression> {
    /// [key](PairNode::parse_key) ~ `:` ~ [value](PairNode::parse_value)
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, key) = input.match_fn(parse_key)?;
        let (state, _) = state.skip(ignore).match_char(':')?;
        let (state, value) = state.skip(ignore).match_fn(parse_value)?;
        state.finish(PairNode { key, value })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(3);
        terms.push(Lisp::function("pair"));
        terms.push(self.key.as_lisp().into());
        terms.push(self.value.lispify().into());
        Lisp::Any(terms)
    }
}

/// - `key ~ : ~ value`
pub fn parse_pair(input: ParseState) -> ParseResult<TableTermNode<ValkyrieExpression>> {
    let (state, term) = PairNode::parse(input)?;
    state.finish(TableTermNode::Pair(term))
}
/// - `term`
pub fn parse_term(input: ParseState) -> ParseResult<TableTermNode<ValkyrieExpression>> {
    let (state, term) = ValkyrieExpression::parse(input)?;
    state.finish(TableTermNode::Item(term))
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

use crate::{expression::ValkyrieExpression, helpers::ignore, traits::ThisParser};
use lispify::{Lisp, Lispify};
use valkyrie_ast::{CallTermNode, IdentifierNode, TableKind, TableNode};
use valkyrie_types::third_party::pex::{BracketPattern, ParseResult, ParseState};

impl From<TableNode<ValkyrieExpression>> for ValkyrieExpression {
    fn from(value: TableNode<ValkyrieExpression>) -> Self {
        ValkyrieExpression::Table(Box::new(value))
    }
}

impl ThisParser for TableNode<ValkyrieExpression> {
    /// `[` ~ `]` | `[` [term](CallTermNode::parse) ( ~ `,` ~ [term](CallTermNode::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, CallTermNode::parse)?;
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

impl ThisParser for CallTermNode<IdentifierNode, ValkyrieExpression> {
    /// - [start]()? ~ `:` ~ [end]()? (~ `:` ~ [step]?)?
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, key) = input.match_optional(parse_key)?;
        let (state, value) = state.skip(ignore).match_fn(ValkyrieExpression::parse)?;
        state.finish(CallTermNode { key, value })
    }

    fn as_lisp(&self) -> Lisp {
        match &self.key {
            Some(key) => Lisp::Any(vec![key.as_lisp().into(), self.value.as_lisp().into()]),
            None => self.value.as_lisp(),
        }
    }
}

/// - `key ~ :`
#[inline]
pub fn parse_key(input: ParseState) -> ParseResult<IdentifierNode> {
    let (state, term) = IdentifierNode::parse(input)?;
    let (state, _) = state.skip(ignore).match_char(':')?;
    state.finish(term)
}

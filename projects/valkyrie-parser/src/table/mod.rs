use crate::{helpers::ignore, traits::ThisParser};
use lispify::{Lisp, Lispify};
use valkyrie_ast::{ApplyTermNode, ExpressionBody, IdentifierNode, TableKind, TableNode};
use valkyrie_types::third_party::pex::{BracketPattern, ParseResult, ParseState};

impl ThisParser for TableNode<ExpressionBody> {
    /// `[` ~ `]` | `[` [term](ApplyTermNode::parse) ( ~ `,` ~ [term](ApplyTermNode::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, ApplyTermNode::parse)?;
        state.finish(TableNode { kind: TableKind::OffsetTable, terms: terms.body, range: state.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::function("table"));
        for term in &self.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for ApplyTermNode<IdentifierNode, ExpressionBody> {
    /// - [start]()? ~ `:` ~ [end]()? (~ `:` ~ [step]?)?
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, key) = input.match_optional(parse_key)?;
        let (state, value) = state.skip(ignore).match_fn(ExpressionBody::parse)?;
        state.finish(ApplyTermNode { key, value })
    }

    fn as_lisp(&self) -> Lisp {
        match &self.key {
            Some(key) => Lisp::Any(vec![key.as_lisp(), self.value.as_lisp()]),
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

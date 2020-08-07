use super::*;
use valkyrie_ast::{NumberLiteralNode, StringLiteralNode, SubscriptNode, TableKeyType, TableTermNode};

impl ThisParser for TableNode {
    /// `[` ~ `]` | `[` [term](CallTermNode::parse) ( ~ `,` ~ [term](CallTermNode::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, TableTermNode::parse)?;
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

impl ThisParser for TableTermNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, pair) = CallTermNode::parse(input)?;
        state.finish(TableTermNode { pair })
    }

    fn as_lisp(&self) -> Lisp {
        self.pair.as_lisp()
    }
}

impl ThisParser for TableKeyType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| IdentifierNode::parse(s).map_inner(|e| TableKeyType::Identifier(Box::new(e))))
            .or_else(|s| NumberLiteralNode::parse(s).map_inner(|e| TableKeyType::Number(Box::new(e))))
            .or_else(|s| StringLiteralNode::parse(s).map_inner(|e| TableKeyType::String(Box::new(e))))
            .or_else(|s| SubscriptNode::parse(s).map_inner(|e| TableKeyType::Subscript(Box::new(e))))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            TableKeyType::Identifier(e) => e.as_lisp(),
            TableKeyType::Number(e) => e.as_lisp(),
            TableKeyType::String(e) => e.as_lisp(),
            TableKeyType::Subscript(e) => e.as_lisp(),
        }
    }
}

impl<K, V> ThisParser for CallTermNode<K, V>
where
    K: ThisParser,
    V: ThisParser,
{
    /// - [start]()? ~ `:` ~ [end]()? (~ `:` ~ [step]?)?
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, key) = input.match_optional(|s| {
            let (state, term) = K::parse(s)?;
            let (state, _) = state.skip(ignore).match_char(':')?;
            state.finish(term)
        })?;
        let (state, value) = state.skip(ignore).match_fn(V::parse)?;
        state.finish(CallTermNode { key, value })
    }

    fn as_lisp(&self) -> Lisp {
        match &self.key {
            Some(key) => Lisp::Any(vec![key.as_lisp(), self.value.as_lisp()]),
            None => self.value.as_lisp(),
        }
    }
}

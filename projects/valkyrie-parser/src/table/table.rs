use super::*;

impl ThisParser for TableNode {
    /// `[` ~ `]` | `[` [term](CallTermNode::parse) ( ~ `,` ~ [term](CallTermNode::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, TableTermNode::parse)?;
        state.finish(TableNode { kind: TableKind::OffsetTable, terms: terms.body, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(self.terms.len() + 2);
        lisp += Lisp::keyword("table");
        for term in &self.terms {
            lisp += term.lispify();
        }
        lisp
    }
}

impl ThisParser for TableTermNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, pair) = CallTermNode::parse(input)?;
        state.finish(TableTermNode { pair })
    }

    fn lispify(&self) -> Lisp {
        self.pair.lispify()
    }
}

impl ThisParser for TableKeyType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose(|s| IdentifierNode::parse(s).map_inner(|e| TableKeyType::Identifier(Box::new(e))))
            .choose(|s| NumberLiteralNode::parse(s).map_inner(|e| TableKeyType::Number(Box::new(e))))
            .choose(|s| StringLiteralNode::parse(s).map_inner(|e| TableKeyType::String(Box::new(e))))
            .choose(|s| SubscriptNode::parse(s).map_inner(|e| TableKeyType::Subscript(Box::new(e))))
            .end_choice()
    }

    fn lispify(&self) -> Lisp {
        match self {
            TableKeyType::Identifier(e) => e.lispify(),
            TableKeyType::Number(e) => e.lispify(),
            TableKeyType::String(e) => e.lispify(),
            TableKeyType::Subscript(e) => e.lispify(),
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

    fn lispify(&self) -> Lisp {
        let value = self.value.lispify();
        match &self.key {
            Some(key) => key.lispify() + value,
            None => value,
        }
    }
}

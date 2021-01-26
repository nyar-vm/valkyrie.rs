use super::*;

impl ThisParser for TupleNode {
    /// `[` ~ `]` | `[` [term](CallTermNode::parse) ( ~ `,` ~ [term](CallTermNode::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, TupleTermNode::parse)?;
        state.finish(TupleNode { kind: TupleKind::List, terms: terms.body, span: get_span(input, state) })
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

impl ThisParser for TupleTermNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, pair) = CallTermNode::parse(input)?;
        state.finish(TupleTermNode { pair })
    }

    fn lispify(&self) -> Lisp {
        self.pair.lispify()
    }
}

impl ThisParser for TupleKeyType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose(|s| IdentifierNode::parse(s).map_inner(|e| TupleKeyType::Identifier(Box::new(e))))
            .choose(|s| NumberLiteralNode::parse(s).map_inner(|e| TupleKeyType::Number(Box::new(e))))
            .choose(|s| StringLiteralNode::parse(s).map_inner(|e| TupleKeyType::String(Box::new(e))))
            .choose(|s| SubscriptCallNode::parse(s).map_inner(|e| TupleKeyType::Subscript(Box::new(e))))
            .end_choice()
    }

    fn lispify(&self) -> Lisp {
        match self {
            TupleKeyType::Identifier(e) => e.lispify(),
            TupleKeyType::Number(e) => e.lispify(),
            TupleKeyType::String(e) => e.lispify(),
            TupleKeyType::Subscript(e) => e.lispify(),
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

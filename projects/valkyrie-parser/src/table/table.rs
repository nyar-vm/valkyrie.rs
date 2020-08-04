use super::*;

impl ThisParser for TableNode {
    /// `[` ~ `]` | `[` [term](MaybePair::parse) ( ~ `,` ~ [term](MaybePair::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input, ignore, MaybePair::parse)?;
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

impl<K, V> ThisParser for MaybePair<K, V>
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
        state.finish(MaybePair { key, value })
    }

    fn as_lisp(&self) -> Lisp {
        match &self.key {
            Some(key) => Lisp::Any(vec![key.as_lisp(), self.value.as_lisp()]),
            None => self.value.as_lisp(),
        }
    }
}

use super::*;
use valkyrie_ast::ExpressionNode;

impl<E> TupleNode<E> {
    #[allow(clippy::wrong_self_convention)]
    pub fn as_table(self) -> TableNode<E> {
        TableNode { kind: TableKind::Tuple, terms: self.terms, range: self.range }
    }
}

// impl From<TupleNode<E>> for ExpressionNode<> {
//
// }

impl<E: ThisParser> ThisParser for TupleNode<E> {
    /// `(` ~ `)` | `(` ~ term ~ , ~ `)` | `(` ~ term ~ , ~ term ( ~ , ~ term)* ~ `)`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")").with_one_tailing(true);
        let (state, terms) = pat.consume(input, ignore, MaybePair::parse)?;
        state.finish(TupleNode { terms: terms.body, range: state.away_from(input) })
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

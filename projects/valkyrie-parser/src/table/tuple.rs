use super::*;
use valkyrie_ast::ExpressionNode;

impl TupleNode {
    #[allow(clippy::wrong_self_convention)]
    pub fn as_table(self) -> TableNode {
        TableNode { kind: TableKind::Tuple, terms: self.terms, span: self.range }
    }
}

// impl From<TupleNode<E>> for ExpressionNode<> {
//
// }

impl ThisParser for TupleNode {
    /// `(` ~ `)` | `(` ~ term ~ , ~ `)` | `(` ~ term ~ , ~ term ( ~ , ~ term)* ~ `)`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")").with_one_tailing(true);
        let (state, terms) = pat.consume(input, ignore, TableTermNode::parse)?;
        state.finish(TupleNode { terms: terms.body, range: get_span(input, state) })
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

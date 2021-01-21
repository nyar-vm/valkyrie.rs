use super::*;

impl TupleNode {
    #[allow(clippy::wrong_self_convention)]
    pub fn as_table(self) -> TupleNode {
        TupleNode { kind: TupleKind::Tuple, terms: self.terms, span: self.span }
    }
}

impl ThisParser for TupleNode {
    /// `(` ~ `)` | `(` ~ term ~ , ~ `)` | `(` ~ term ~ , ~ term ( ~ , ~ term)* ~ `)`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")").with_one_tailing(true);
        let (state, terms) = pat.consume(input, ignore, TupleTermNode::parse)?;
        state.finish(TupleNode { terms: terms.body, span: get_span(input, state) })
    }
}

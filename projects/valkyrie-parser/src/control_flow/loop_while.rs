use super::*;

impl ThisParser for WhileLoop {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("while")?;
        let (state, condition) = state.skip(ignore).match_fn(ConditionType::parse)?;
        let (state, stmts) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        let (finally, rest) = state.skip(ignore).match_optional(ElsePart::parse)?;
        finally.finish(WhileLoop { condition, body: stmts, r#else: rest, span: get_span(input, finally) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.body.terms.len() + 1);
        terms.push(Lisp::keyword("loop"));
        terms.push(self.condition.as_lisp());
        for term in &self.body.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

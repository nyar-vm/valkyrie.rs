use super::*;

impl ThisParser for WhileLoopNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("while")?;
        let (state, condition) = state.skip(ignore).match_fn(ConditionType::parse)?;
        let (state, stmts) = state.skip(ignore).match_fn(FunctionBodyPart::parse)?;
        let (finally, rest) = state.skip(ignore).match_optional(ElsePart::parse)?;
        finally.finish(WhileLoopNode {
            condition,
            body: stmts.body.to_vec(),
            r#else: rest.map(|v| v.body.to_vec()).unwrap_or_default(),
            span: get_span(input, finally),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.body.len() + 1);
        terms.push(Lisp::keyword("loop"));
        terms.push(self.condition.as_lisp());
        for term in &self.body {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

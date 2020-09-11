use super::*;

impl ThisParser for ForLoop {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("for")?;
        let (state, pattern) = state.skip(ignore).match_fn(PatternExpression::parse)?;
        let (state, _) = state.skip(ignore).match_fn(parse_in)?;
        let (state, expr) = state.skip(ignore).match_fn(|s| parse_expression_node(s, ExpressionContext::default()))?;
        let (state, cond) = state.skip(ignore).match_optional(PatternGuard::parse)?;
        let (state, body) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        let (state, other) = state.skip(ignore).match_optional(ElseStatement::parse)?;
        state.finish(ForLoop { pattern, iterator: expr, condition: cond, body, r#else: other, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(10);
        terms.push(Lisp::keyword("for"));
        terms.push(self.pattern.as_lisp());
        terms.push(Lisp::keyword("in"));
        terms.push(self.iterator.as_lisp());
        if let Some(cond) = &self.condition {
            terms.push(Lisp::keyword("if"));
            terms.push(cond.as_lisp());
        }
        terms.push(Lisp::Any(self.body.terms.iter().map(|s| s.as_lisp()).collect()));
        terms.push(Lisp::keyword("else"));
        terms.push(Lisp::Any(self.r#else.iter().map(|s| s.as_lisp()).collect()));
        Lisp::Any(terms)
    }
}

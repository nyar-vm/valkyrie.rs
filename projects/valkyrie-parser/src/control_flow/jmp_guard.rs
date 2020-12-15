use super::*;

impl ThisParser for GuardStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("guard")?;
        let (state, cond) = state.skip(ignore).match_fn(ExpressionNode::parse)?;
        let (finally, body) = state.skip(ignore).match_fn(GuardStatementBody::parse)?;
        finally.finish(GuardStatement { condition: cond, main_body: body, span: get_span(input, finally) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(4);

        match &self.main_body {
            GuardStatementBody::Positive(v) => {
                lisp += Lisp::keyword("guard/positive");
                lisp += self.condition.as_lisp();
                lisp.extend(v.body.terms.iter().map(|s| s.as_lisp()));
            }
            GuardStatementBody::Negative(v) => {
                lisp += Lisp::keyword("guard/negative");
                lisp += self.condition.as_lisp();
                lisp.extend(v.body.terms.iter().map(|s| s.as_lisp()));
            }
        }
        lisp
    }
}

impl ThisParser for GuardStatementBody {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose(|s| parse_maybe_then(s).map_inner(GuardStatementBody::Positive))
            .choose(|s| ElseStatement::parse(s).map_inner(GuardStatementBody::Negative))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            GuardStatementBody::Positive(v) => v.as_lisp(),
            GuardStatementBody::Negative(v) => v.as_lisp(),
        }
    }
}

impl ThisParser for GuardLetStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("guard")?;
        let (state, _) = state.skip(ignore).match_str("let")?;
        let (state, pat) = state.skip(ignore).match_fn(PatternExpressionType::parse)?;
        let (state, _) = state.skip(ignore).match_str("=")?;
        let (state, expr) = state.skip(ignore).match_fn(ExpressionNode::parse)?;
        let (finally, body) = state.skip(ignore).match_fn(GuardStatementBody::parse)?;
        finally.finish(GuardLetStatement { pattern: pat, expression: expr, main_body: body, span: get_span(input, finally) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("guard/cases");
        lisp += self.pattern.as_lisp();
        lisp += self.expression.as_lisp();
        lisp += self.main_body.as_lisp();
        lisp
    }
}

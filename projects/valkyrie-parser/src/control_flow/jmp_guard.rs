use super::*;

impl ThisParser for GuardStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("guard")?;
        let (state, cond) = state.skip(ignore).match_fn(ExpressionNode::parse)?;
        let (finally, body) = state.skip(ignore).match_fn(GuardPattern::parse)?;
        finally.finish(GuardStatement { condition: cond, main_body: body, span: get_span(input, finally) })
    }
}

impl ThisParser for GuardPattern {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose(|s| parse_maybe_then(s).map_inner(GuardPattern::Expression))
            .choose(|s| ElseStatement::parse(s).map_inner(GuardPattern::List))
            .end_choice()
    }

    fn lispify(&self) -> Lisp {
        match self {
            GuardPattern::Expression(v) => v.lispify(),
            GuardPattern::List(v) => v.lispify(),
        }
    }
}

impl ThisParser for GuardLetStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("guard")?;
        let (state, _) = state.skip(ignore).match_str("let")?;
        let (state, pat) = state.skip(ignore).match_fn(LetPattern::parse)?;
        let (state, _) = state.skip(ignore).match_str("=")?;
        let (state, expr) = state.skip(ignore).match_fn(ExpressionNode::parse)?;
        let (finally, body) = state.skip(ignore).match_fn(GuardPattern::parse)?;
        finally.finish(GuardLetStatement { pattern: pat, expression: expr, main_body: body, span: get_span(input, finally) })
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("guard/cases");
        lisp += self.pattern.lispify();
        lisp += self.expression.lispify();
        lisp += self.main_body.lispify();
        lisp
    }
}

use super::*;

impl ThisParser for IfStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = parse_when(input)?;
        let (state, cond) = state.skip(ignore).match_fn(IfConditionNode::parse)?;
        let (state, body) = state.match_repeats(parse_else_if)?;
        let (finally, else_body) = state.skip(ignore).match_optional(ElseStatement::parse)?;
        let mut branches = vec![cond];
        branches.extend(body);
        finally.finish(IfStatement { branches, else_body, span: get_span(input, finally) })
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(10);
        lisp += Lisp::keyword("branches");
        for branch in &self.branches {
            lisp += branch.lispify();
        }
        if let Some(else_body) = &self.else_body {
            lisp += else_body.lispify();
        }
        lisp
    }
}

impl ThisParser for IfLetStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("if")?;
        let (state, _) = state.skip(ignore).match_str("let")?;
        let (state, pat) = state.skip(ignore).match_fn(PatternExpressionType::parse)?;
        let (state, _) = state.skip(ignore).match_str("=")?;
        let (state, expr) = state.skip(ignore).match_fn(ExpressionNode::parse)?;
        let (finally, mut then_body) = state.skip(ignore).match_fn(parse_maybe_then)?;
        then_body.show = false;
        let (finally, else_body) = finally.skip(ignore).match_optional(ElseStatement::parse)?;
        finally.finish(Self { pattern: pat, expression: expr, then_body, else_body, span: get_span(input, finally) })
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(10);
        lisp += Lisp::keyword("pattern");
        lisp += self.pattern.lispify();
        lisp += Lisp::keyword("expression");
        lisp += self.expression.lispify();
        lisp += self.then_body.lispify();
        if let Some(else_body) = &self.else_body {
            lisp += else_body.lispify();
        }
        lisp
    }
}

fn parse_else_if(input: ParseState) -> ParseResult<IfConditionNode> {
    let (state, _) = str("else")(input.skip(ignore))?;
    let (state, _) = str("if")(state.skip(ignore))?;
    let (state, cond) = state.skip(ignore).match_fn(IfConditionNode::parse)?;
    state.finish(cond)
}

impl ThisParser for IfConditionNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, cond) = input.match_fn(ExpressionNode::parse)?;
        let (state, body) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        state.finish(IfConditionNode { condition: cond, body, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        self.condition.lispify() + self.body.lispify()
    }
}
impl ThisParser for ThenStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("then")?;
        let (state, func) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        state.finish(ThenStatement { show: true, body: func, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword("then") + self.body.lispify()
    }
}

impl ThisParser for ElseStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("else")?;
        let (state, func) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        state.finish(ElseStatement { body: func, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword("else") + self.body.lispify()
    }
}

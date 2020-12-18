use super::*;

impl ThisParser for LetBindNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("let")?;
        let (state, pattern) = state.skip(ignore).match_fn(PatternExpressionType::parse)?;
        let (state, type_hint) = state.match_optional(parse_type_hint)?;
        let (state, body) = state.match_optional(parse_expr)?;
        state.finish(LetBindNode { pattern, type_hint, body })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(5);
        lisp += Lisp::keyword("let");
        lisp += self.pattern.as_lisp();
        if let Some(type_hint) = &self.type_hint {
            lisp += type_hint.as_lisp();
        }
        if let Some(body) = &self.body {
            lisp += body.as_lisp();
        }
        lisp
    }
}

fn parse_type_hint(input: ParseState) -> ParseResult<ExpressionNode> {
    let (state, _) = input.skip(ignore).match_str(":")?;
    let (state, expr) = state.skip(ignore).match_fn(TypingExpression::parse)?;
    state.finish(expr.as_normal())
}

fn parse_expr(input: ParseState) -> ParseResult<ExpressionNode> {
    let (state, _) = input.skip(ignore).match_str("=")?;
    let (state, expr) = state.skip(ignore).match_fn(ExpressionNode::parse)?;
    state.finish(expr)
}

use super::*;

impl ThisParser for LetBindNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("let")?;
        let (state, pattern) = state.skip(ignore).match_fn(PatternType::parse)?;
        let (state, type_hint) = state.match_optional(parse_type_hint)?;
        let (state, body) = state.match_optional(parse_expr)?;
        state.finish(LetBindNode { pattern, type_hint, body })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(5);
        terms.push(Lisp::keyword("let"));
        terms.push(self.pattern.as_lisp());
        if let Some(type_hint) = &self.type_hint {
            terms.push(type_hint.as_lisp());
        }
        if let Some(body) = &self.body {
            terms.push(body.as_lisp());
        }
        Lisp::Any(terms)
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

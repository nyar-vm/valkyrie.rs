use super::*;

impl ThisParser for ControlNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, kw) = ControlType::parse(input)?;
        let (state, expr) = state.skip(ignore).match_optional(ExpressionNode::parse)?;
        state.finish(ControlNode { r#type: kw, expression: expr, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(2);
        terms.push(self.r#type.as_lisp());
        if let Some(s) = &self.expression {
            terms.push(s.as_lisp());
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for ControlType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| s.match_str("break").map_inner(|_| ControlType::Break))
            .or_else(|s| s.match_str("continue").map_inner(|_| ControlType::Continue))
            .or_else(|s| s.match_str("fallthrough").map_inner(|_| ControlType::Fallthrough))
            .or_else(|s| s.match_str("return").map_inner(|_| ControlType::Return))
            .or_else(|s| s.match_str("raise").map_inner(|_| ControlType::Raise))
            .or_else(|s| {
                let (state, _) = s.match_str("yield")?;
                state.match_optional(parse_yield).map_inner(|s| s.unwrap_or(ControlType::YieldReturn))
            })
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::keyword(self.as_str())
    }
}

fn parse_yield(input: ParseState) -> ParseResult<ControlType> {
    input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| s.match_str("return").map_inner(|_| ControlType::YieldReturn))
        .or_else(|s| s.match_str("from").map_inner(|_| ControlType::YieldFrom))
        .or_else(|s| s.match_str("break").map_inner(|_| ControlType::YieldBreak))
        .end_choice()
}

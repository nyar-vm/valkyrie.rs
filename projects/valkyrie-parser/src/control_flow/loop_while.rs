use super::*;

impl ThisParser for WhileLoop {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, kw) = WhileLoopKind::parse(input)?;
        let (state, condition) = state.skip(ignore).match_fn(WhileConditionNode::parse)?;
        let (state, stmts) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        let (finally, rest) = state.skip(ignore).match_optional(OtherwiseStatement::parse)?;
        finally.finish(WhileLoop { kind: kw, condition, then_body: stmts, otherwise: rest, span: get_span(input, finally) })
    }
}

impl ThisParser for WhileLoopKind {
    fn parse(input: ParseState) -> ParseResult<Self> {
        if input.residual.starts_with("while") {
            input.finish(Self::While)
        }
        else if input.residual.starts_with("until") {
            input.finish(Self::Until)
        }
        else {
            StopBecause::missing_string("while or until", input.start_offset)?
        }
    }

    fn lispify(&self) -> Lisp {
        match self {
            Self::While => Lisp::keyword("while"),
            Self::Until => Lisp::keyword("until"),
        }
    }
}

impl ThisParser for OtherwiseStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("otherwise")?;
        let (state, func) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        state.finish(Self { terms: func.terms, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword("otherwise") + self.terms.iter().map(|s| s.lispify()).collect::<Lisp>()
    }
}

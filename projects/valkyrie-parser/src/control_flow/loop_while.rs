use super::*;
use pex::StopBecause;
use valkyrie_ast::{OtherwiseStatement, WhileLoopKind};

impl ThisParser for WhileLoop {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, kw) = WhileLoopKind::parse(input)?;
        let (state, condition) = state.skip(ignore).match_fn(WhileConditionNode::parse)?;
        let (state, stmts) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        let (finally, rest) = state.skip(ignore).match_optional(ElseStatement::parse)?;
        finally.finish(WhileLoop { kind: kw, condition, then_body: stmts, else_body: rest, span: get_span(input, finally) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(self.then_body.terms.len() + 1);
        lisp += self.kind.as_lisp();
        lisp += self.condition.as_lisp();
        for term in &self.then_body.terms {
            lisp += term.as_lisp();
        }
        lisp
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

    fn as_lisp(&self) -> Lisp {
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

    fn as_lisp(&self) -> Lisp {
        Lisp::keyword("otherwise") + self.terms.iter().map(|s| s.as_lisp()).collect::<Lisp>()
    }
}

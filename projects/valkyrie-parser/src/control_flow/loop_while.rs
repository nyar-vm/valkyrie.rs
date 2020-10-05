use super::*;
use pex::StopBecause;
use valkyrie_ast::WhileLoopKind;

impl ThisParser for WhileLoop {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, kw) = WhileLoopKind::parse(input)?;
        let (state, condition) = state.skip(ignore).match_fn(WhileConditionNode::parse)?;
        let (state, stmts) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        let (finally, rest) = state.skip(ignore).match_optional(ElseStatement::parse)?;
        finally.finish(WhileLoop { kind: kw, condition, then_body: stmts, else_body: rest, span: get_span(input, finally) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.then_body.terms.len() + 1);
        terms.push(Lisp::keyword("loop"));
        terms.push(self.condition.as_lisp());
        for term in &self.then_body.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
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

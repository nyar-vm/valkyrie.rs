use super::*;
use valkyrie_ast::GuardStatementBody;

impl ThisParser for GuardStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("guard")?;
        let (state, cond) = state.skip(ignore).match_fn(ExpressionNode::parse)?;
        let (finally, body) = state.skip(ignore).match_fn(GuardStatementBody::parse)?;
        finally.finish(GuardStatement { condition: cond, main_body: body, span: get_span(input, finally) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
        // let mut terms = Vec::with_capacity(5);
        // terms.push(Lisp::keyword("guard"));
        // terms.push(self.condition.as_lisp());
        // terms.push(Lisp::keyword("else"));
        // terms.push(self.body.as_lisp());
        // lisp
    }
}

impl ThisParser for GuardStatementBody {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

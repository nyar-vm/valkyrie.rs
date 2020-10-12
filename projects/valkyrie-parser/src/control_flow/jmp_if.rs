use super::*;
use valkyrie_ast::{IfLetStatement, ThenStatement};

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

    fn as_lisp(&self) -> Lisp {
        todo!()
        // let mut terms = Vec::with_capacity(10);
        // terms.push(Lisp::keyword("branches"));
        // for branch in &self.branches {
        //     terms.push(branch.as_lisp());
        // }
        // if let Some(else_body) = &self.else_body {
        //     terms.push(else_body.as_lisp());
        // }
        // lisp
    }
}

impl ThisParser for IfLetStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
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

    fn as_lisp(&self) -> Lisp {
        todo!()
        // let mut terms = Vec::with_capacity(10);
        // terms.push(self.condition.as_lisp());
        // terms.push(self.body.as_lisp());
        // lisp
    }
}
impl ThisParser for ThenStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("then")?;
        let (state, func) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        state.finish(ThenStatement { show: true, body: func, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::keyword("then") + self.body.as_lisp()
    }
}

impl ThisParser for ElseStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("else")?;
        let (state, func) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        state.finish(ElseStatement { body: func, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::keyword("else") + self.body.as_lisp()
    }
}
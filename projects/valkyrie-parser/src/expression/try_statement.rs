use super::*;
use valkyrie_ast::{StatementBlock, TryStatement};

impl ThisParser for TryStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("try")?;
        let (state, catch) = state.skip(ignore).match_optional(|s| parse_expression_node(s, ExpressionContext::in_type()))?;
        let (state, block) = state.skip(ignore).match_fn(StatementBlock::parse)?;

        state.finish(Self { handler: catch, body: block, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

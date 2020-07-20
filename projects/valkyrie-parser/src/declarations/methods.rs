use super::*;
use crate::utils::parse_expression_node;
use valkyrie_ast::{ExpressionContext, ExpressionNode, FunctionCommonPart, GenericArgumentNode};

impl ThisParser for FunctionCommonPart {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, generic) = input.match_optional(GenericArgumentNode::parse)?;
        let (state, args) = state.skip(ignore).match_fn(ApplyArgumentNode::parse)?;
        let (state, ret) = state.skip(ignore).match_optional(parse_return_type)?;
        let (finally, body) = state.skip(ignore).match_optional(FunctionBody::parse)?;
        finally.finish(FunctionCommonPart { generic, arguments: args, r#return: ret, body: body.map(|s| s.body) })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

#[inline]
fn parse_return_type(input: ParseState) -> ParseResult<ExpressionNode> {
    let (state, _) = input.begin_choice().or_else(|s| s.match_str("->")).or_else(|s| s.match_str(":")).end_choice()?;
    let (state, typing) = parse_expression_node(state.skip(ignore), ExpressionContext::in_type())?;
    state.finish(typing)
}

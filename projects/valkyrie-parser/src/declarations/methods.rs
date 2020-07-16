use super::*;
use valkyrie_ast::CommonFunctionPart;

impl ThisParser for CommonFunctionPart {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, args) = input
            .skip(ignore)
            .match_fn(ApplyArgumentNode::<ExpressionNode<TypeLevelExpressionType>, ExpressionNode<ExpressionType>>::parse)?;
        let (state, ret) = state.skip(ignore).match_optional(parse_return_type)?;

        let (finally, body) = state.skip(ignore).match_fn(FunctionBody::parse)?;
        finally.finish(CommonFunctionPart {
            arguments: args.map_value(|s| ExpressionNode {
                context: ExpressionContext::Type,
                expression: s.expression.wrapper,
                range: s.range,
            }),
            r#return: ret,
            body: body.body,
        })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

#[inline]
fn parse_return_type(input: ParseState) -> ParseResult<ExpressionNode<ExpressionType>> {
    let (state, _) = input.begin_choice().or_else(|s| s.match_str("->")).or_else(|s| s.match_str(":")).end_choice()?;
    let (state, typing) = state.skip(ignore).match_fn(ExpressionNode::<TypeLevelExpressionType>::parse)?;
    state.finish(typing.map(|s| s.wrapper).with_context(ExpressionContext::Type))
}

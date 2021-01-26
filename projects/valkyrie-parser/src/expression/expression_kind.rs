use super::*;
use valkyrie_ast::GuardStatement;

impl ThisParser for ExpressionNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        parse_expression_node(input, ExpressionContext::default())
    }

    fn lispify(&self) -> Lisp {
        self.body.lispify()
    }
}

impl ThisParser for ExpressionType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        parse_expression_body(input, ExpressionContext::default())
    }
}

impl ExpressionStream {
    /// term (~ infix ~ term)*
    /// 1 + (1 + +3? + 4)
    pub fn parse(input: ParseState, ctx: ExpressionContext) -> ParseResult<Vec<ExpressionStream>> {
        let mut stream = Vec::with_capacity(4);
        let (state, _) = input.match_fn(|s| parse_term(s, &mut stream, ctx))?;
        let (state, _) = state.match_repeats(|s| parse_infix_term(s, &mut stream, ctx))?;
        state.finish(stream)
    }
}

/// `~ infix ~ term`
#[inline(always)]
fn parse_infix_term<'i>(
    input: ParseState<'i>,
    stream: &mut Vec<ExpressionStream>,
    ctx: ExpressionContext,
) -> ParseResult<'i, ()> {
    let (state, infix) = ValkyrieInfix::parse(input.skip(ignore), ctx.type_level)?;
    stream.push(ExpressionStream::Infix(infix));
    let (state, _) = state.skip(ignore).match_fn(|s| parse_term(s, stream, ctx))?;
    state.finish(())
}

/// `( ~ term ~ )`
pub fn parse_group(input: ParseState, ctx: ExpressionContext) -> ParseResult<Vec<ExpressionStream>> {
    let (state, _) = input.match_char('(')?;
    let (state, group) = state.skip(ignore).match_fn(|s| ExpressionStream::parse(s, ctx))?;
    let (state, _) = state.skip(ignore).match_char(')')?;
    // Only join the global stream after all success
    state.finish(group)
}

/// `(~ prefix)* ~ value (~ suffix)*`
fn parse_term<'i>(state: ParseState<'i>, stream: &mut Vec<ExpressionStream>, ctx: ExpressionContext) -> ParseResult<'i, ()> {
    let (state, _) = state.match_repeats(|s| parse_prefix(s, stream))?;
    let (state, _) = parse_expr_value(state, stream, ctx)?;
    let (state, _) = state.match_repeats(|s| parse_suffix(s, stream, ctx))?;
    state.finish(())
}

#[inline(always)]
fn parse_prefix<'a>(input: ParseState<'a>, stream: &mut Vec<ExpressionStream>) -> ParseResult<'a, ()> {
    let (state, prefix) = input.skip(ignore).match_fn(ValkyriePrefix::parse)?;
    stream.push(ExpressionStream::Prefix(prefix));
    state.finish(())
}

#[inline(always)]
fn parse_suffix<'a>(input: ParseState<'a>, stream: &mut Vec<ExpressionStream>, ctx: ExpressionContext) -> ParseResult<'a, ()> {
    let (state, suffix) = input.skip(ignore).match_fn(|s| ValkyrieSuffix::parse(s, ctx.type_level))?;
    stream.push(ExpressionStream::Postfix(suffix));
    state.finish(())
}

#[inline]
fn parse_expr_value<'a>(
    input: ParseState<'a>,
    stream: &mut Vec<ExpressionStream>,
    ctx: ExpressionContext,
) -> ParseResult<'a, ()> {
    let (state, term) = input
        .skip(ignore)
        .begin_choice()
        .choose(|s| parse_group(s, ctx).map_inner(ExpressionStream::Group))
        .choose(|s| parse_expression(s, ctx.allow_curly).map_inner(ExpressionStream::Term))
        .end_choice()?;

    stream.push(term);
    state.finish(())
}

pub fn parse_expression(input: ParseState, allow_curly: bool) -> ParseResult<ExpressionType> {
    let (state, mut base) = input
        .begin_choice()
        .choose_from(NewConstructNode::parse)
        .choose_from(NumberLiteralNode::parse)
        .choose_from(StringLiteralNode::parse)
        .choose_from(LambdaSlotNode::parse)
        .choose_from(GuardStatement::parse)
        .choose_from(IfStatement::parse)
        .choose_from(SwitchStatement::parse)
        .choose_from(TryStatement::parse)
        .choose_from(RaiseNode::parse)
        .choose_from(NamePathNode::parse)
        .choose_from(TupleNode::parse)
        .choose_from(TupleNode::parse)
        .end_choice()?;
    let (state, rest) = match allow_curly {
        true => state.match_repeats(parse_postfix_curly),
        false => state.match_repeats(PostfixCallPart::parse),
    }?;
    for caller in rest {
        match caller {
            PostfixCallPart::Apply(v) => base = ExpressionType::call_apply(base, v),
            PostfixCallPart::ApplyDot(v) => base = ExpressionType::dot_apply(base, v),
            PostfixCallPart::View(v) => base = ExpressionType::call_subscript(base, v),
            PostfixCallPart::Generic(v) => base = ExpressionType::call_generic(base, v),
            PostfixCallPart::Lambda(v) => base = ExpressionType::call_lambda(base, v),
            PostfixCallPart::LambdaDot(v) => base = ExpressionType::dot_lambda(base, v),
            PostfixCallPart::Match(v) => base = ExpressionType::dot_match(base, v),
        }
    }
    state.finish(base)
}

impl ThisParser for PostfixCallPart {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .skip(ignore)
            .begin_choice()
            .choose(|s| ApplyCallNode::parse(s).map_into())
            .choose(|s| ApplyDotNode::parse(s).map_into())
            .choose(|s| SubscriptCallNode::parse(s).map_into())
            .choose(|s| GenericCallNode::parse(s).map_into())
            .end_choice()
    }

    fn lispify(&self) -> Lisp {
        unreachable!()
    }
}

fn parse_postfix_curly(input: ParseState) -> ParseResult<PostfixCallPart> {
    input
        .skip(ignore)
        .begin_choice()
        .choose_from(MatchDotStatement::parse)
        .choose(|s| ApplyCallNode::parse(s).map_into())
        .choose(|s| ApplyDotNode::parse(s).map_into())
        .choose(|s| SubscriptCallNode::parse(s).map_into())
        .choose(|s| GenericCallNode::parse(s).map_into())
        .choose(|s| ClosureCallNode::parse(s).map_into())
        .choose(|s| LambdaDotNode::parse(s).map_into())
        .end_choice()
}

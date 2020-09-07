use super::*;
use valkyrie_ast::{IfStatement, LambdaSlotNode};

impl ThisParser for PrefixNode {
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::operator(self.operator.kind.as_str(), &[self.base.as_lisp()])
    }
}

impl ThisParser for InfixNode {
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::operator(self.operator.kind.as_str(), &[self.lhs.as_lisp(), self.rhs.as_lisp()])
    }
}

impl ThisParser for PostfixNode {
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::operator(self.operator.kind.as_str(), &[self.base.as_lisp()])
    }
}

impl ThisParser for ExpressionNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        parse_expression_node(input, ExpressionContext::default())
    }

    fn as_lisp(&self) -> Lisp {
        self.body.as_lisp()
    }
}

impl ThisParser for ExpressionBody {
    fn parse(input: ParseState) -> ParseResult<Self> {
        parse_expression_body(input, ExpressionContext::default())
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            Self::Placeholder => Lisp::Keyword("placeholder".into()),
            Self::Prefix(v) => v.as_lisp(),
            Self::Binary(v) => v.as_lisp(),
            Self::Suffix(v) => v.as_lisp(),
            Self::Number(v) => v.as_lisp(),
            Self::Symbol(v) => v.as_lisp(),
            Self::String(v) => v.as_lisp(),
            Self::Table(v) => v.as_lisp(),
            Self::Apply(v) => v.as_lisp(),
            Self::ApplyDot(v) => v.as_lisp(),
            Self::Subscript(v) => v.as_lisp(),
            Self::GenericCall(v) => v.as_lisp(),
            Self::LambdaCall(v) => v.as_lisp(),
            Self::LambdaDot(v) => v.as_lisp(),
            Self::New(v) => v.as_lisp(),
            Self::Control(v) => v.as_lisp(),
            Self::If(v) => v.as_lisp(),
            Self::Slot(v) => v.as_lisp(),
        }
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
    let (state, _) = state.match_repeats(|s| parse_suffix(s, stream))?;
    state.finish(())
}

#[inline(always)]
fn parse_prefix<'a>(input: ParseState<'a>, stream: &mut Vec<ExpressionStream>) -> ParseResult<'a, ()> {
    let (state, prefix) = input.skip(ignore).match_fn(ValkyriePrefix::parse)?;
    stream.push(ExpressionStream::Prefix(prefix));
    state.finish(())
}

#[inline(always)]
fn parse_suffix<'a>(input: ParseState<'a>, stream: &mut Vec<ExpressionStream>) -> ParseResult<'a, ()> {
    let (state, suffix) = input.skip(ignore).match_fn(ValkyrieSuffix::parse)?;
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
        .or_else(|s| parse_group(s, ctx).map_inner(ExpressionStream::Group))
        .or_else(|s| parse_expression(s, ctx.allow_curly).map_inner(ExpressionStream::Term))
        .end_choice()?;

    stream.push(term);
    state.finish(())
}

pub fn parse_expression(input: ParseState, allow_curly: bool) -> ParseResult<ExpressionBody> {
    let (state, mut base) = input
        .begin_choice()
        .or_else(|s| NewConstructNode::parse(s).map_inner(Into::into))
        .or_else(|s| NumberLiteralNode::parse(s).map_inner(Into::into))
        .or_else(|s| StringLiteralNode::parse(s).map_inner(Into::into))
        .or_else(|s| LambdaSlotNode::parse(s).map_inner(Into::into))
        .or_else(|s| IfStatement::parse(s).map_inner(Into::into))
        .or_else(|s| NamePathNode::parse(s).map_inner(Into::into))
        .or_else(|s| TableNode::parse(s).map_inner(Into::into))
        .or_else(|s| TupleNode::parse(s).map_inner(Into::into))
        .end_choice()?;
    let (state, rest) = match allow_curly {
        true => state.match_repeats(parse_postfix_curly),
        false => state.match_repeats(parse_postfix),
    }?;
    for caller in rest {
        match caller {
            PostfixCallPart::Apply(v) => base = ExpressionBody::call_apply(base, v),
            PostfixCallPart::ApplyDot(v) => base = ExpressionBody::dot_apply(base, v),
            PostfixCallPart::View(v) => base = ExpressionBody::call_subscript(base, v),
            PostfixCallPart::Generic(v) => base = ExpressionBody::call_generic(base, v),
            PostfixCallPart::Lambda(v) => base = ExpressionBody::call_lambda(base, v),
            PostfixCallPart::LambdaDot(v) => base = ExpressionBody::dot_lambda(base, v),
        }
    }
    state.finish(base)
}

fn parse_postfix(input: ParseState) -> ParseResult<PostfixCallPart> {
    input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| ApplyCallNode::parse(s).map_inner(Into::into))
        .or_else(|s| ApplyDotNode::parse(s).map_inner(Into::into))
        .or_else(|s| SubscriptNode::parse(s).map_inner(Into::into))
        .or_else(|s| GenericCallNode::parse(s).map_inner(Into::into))
        .end_choice()
}

fn parse_postfix_curly(input: ParseState) -> ParseResult<PostfixCallPart> {
    input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| ApplyCallNode::parse(s).map_inner(Into::into))
        .or_else(|s| ApplyDotNode::parse(s).map_inner(Into::into))
        .or_else(|s| SubscriptNode::parse(s).map_inner(Into::into))
        .or_else(|s| GenericCallNode::parse(s).map_inner(Into::into))
        .or_else(|s| LambdaCallNode::parse(s).map_inner(Into::into))
        .or_else(|s| LambdaDotNode::parse(s).map_inner(Into::into))
        .end_choice()
}

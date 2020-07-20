use super::*;
use valkyrie_ast::{ExpressionTermNode, ExpressionTypeNode};

impl ThisParser for PrefixNode<ExpressionBody> {
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::operator(self.operator.to_string(), &[self.body.as_lisp()])
    }
}

impl ThisParser for InfixNode<ExpressionBody> {
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::operator(self.operator.to_string(), &[self.lhs.as_lisp(), self.rhs.as_lisp()])
    }
}

impl ThisParser for PostfixNode<ExpressionBody> {
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::operator(self.operator.to_string(), &[self.body.as_lisp()])
    }
}

impl ThisParser for ExpressionTermNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let resolver = ExpressionResolver::default();
        let (state, stream) = ExpressionStream::parse(input, false)?;
        let body = resolver.resolve(stream)?;
        state.finish(ExpressionNode { body, range: state.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        self.body.as_lisp()
    }
}

impl ThisParser for ExpressionTypeNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let resolver = ExpressionResolver::default();
        let (state, stream) = ExpressionStream::parse(input, true)?;
        let body = resolver.resolve(stream)?;
        state.finish(ExpressionNode { body, range: state.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        self.body.as_lisp()
    }
}

impl ThisParser for ExpressionBody {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let resolver = ExpressionResolver::default();
        let (state, stream) = ExpressionStream::parse(input, false)?;
        state.finish(resolver.resolve(stream)?)
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            ExpressionBody::Placeholder => Lisp::Keyword("placeholder".into()),
            ExpressionBody::Prefix(v) => v.as_lisp(),
            ExpressionBody::Binary(v) => v.as_lisp(),
            ExpressionBody::Suffix(v) => v.as_lisp(),
            ExpressionBody::Number(v) => v.as_lisp(),
            ExpressionBody::Symbol(v) => v.as_lisp(),
            ExpressionBody::String(v) => v.as_lisp(),
            ExpressionBody::Table(v) => v.as_lisp(),
            ExpressionBody::Apply(v) => v.as_lisp(),
            ExpressionBody::ApplyDot(v) => v.as_lisp(),
            ExpressionBody::View(v) => v.as_lisp(),
            ExpressionBody::GenericCall(v) => v.as_lisp(),
        }
    }
}

impl ExpressionStream {
    /// term (~ infix ~ term)*
    /// 1 + (1 + +3? + 4)
    pub fn parse(state: ParseState, type_level: bool) -> ParseResult<Vec<ExpressionStream>> {
        let mut stream = Vec::with_capacity(4);
        let (state, _) = state.match_fn(|s| parse_term(s, &mut stream, type_level))?;
        let (state, _) = state.match_repeats(|s| parse_infix_term(s, &mut stream, type_level))?;
        state.finish(stream)
    }
}

/// `~ infix ~ term`
#[inline(always)]
fn parse_infix_term<'i>(input: ParseState<'i>, stream: &mut Vec<ExpressionStream>, type_level: bool) -> ParseResult<'i, ()> {
    let (state, infix) = ValkyrieInfix::parse(input.skip(ignore), type_level)?;
    stream.push(ExpressionStream::Infix(infix));
    let (state, _) = state.skip(ignore).match_fn(|s| parse_term(s, stream, type_level))?;
    state.finish(())
}

/// `( ~ term ~ )`
pub fn parse_group(input: ParseState, type_level: bool) -> ParseResult<Vec<ExpressionStream>> {
    let (state, _) = input.match_char('(')?;
    let (state, group) = state.skip(ignore).match_fn(|s| ExpressionStream::parse(s, type_level))?;
    let (state, _) = state.skip(ignore).match_char(')')?;
    // Only join the global stream after all success
    state.finish(group)
}

/// `(~ prefix)* ~ value (~ suffix)*`
fn parse_term<'i>(state: ParseState<'i>, stream: &mut Vec<ExpressionStream>, type_level: bool) -> ParseResult<'i, ()> {
    let (state, _) = state.match_repeats(|s| parse_prefix(s, stream))?;
    let (state, _) = parse_expr_value(state, stream, type_level)?;
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
fn parse_expr_value<'a>(input: ParseState<'a>, stream: &mut Vec<ExpressionStream>, type_level: bool) -> ParseResult<'a, ()> {
    let (state, term) = input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| parse_group(s, type_level).map_inner(ExpressionStream::Group))
        .or_else(|s| parse_value(s).map_inner(ExpressionStream::Term))
        .end_choice()?;

    stream.push(term);
    state.finish(())
}

pub enum NormalPostfixCall {
    Apply(Box<ApplyCallNode<ExpressionBody>>),
    ApplyDot(Box<ApplyDotNode<ExpressionBody>>),
    View(Box<ViewNode<ExpressionBody>>),
    Generic(Box<GenericCall<ExpressionBody>>),
}

#[inline]
pub fn parse_value(input: ParseState) -> ParseResult<ExpressionBody> {
    let (state, mut base) = input
        .begin_choice()
        .or_else(|s| NamePathNode::parse(s).map_inner(Into::into))
        .or_else(|s| NumberLiteralNode::parse(s).map_inner(Into::into))
        .or_else(|s| StringLiteralNode::parse(s).map_inner(Into::into))
        .or_else(|s| TableNode::parse(s).map_inner(Into::into))
        .end_choice()?;
    let (state, rest) = state.match_repeats(NormalPostfixCall::parse)?;
    for caller in rest {
        match caller {
            NormalPostfixCall::Apply(v) => base = ExpressionBody::Apply(v.rebase(base)),
            NormalPostfixCall::ApplyDot(v) => base = ExpressionBody::ApplyDot(v.rebase(base)),
            NormalPostfixCall::View(v) => base = ExpressionBody::View(v.rebase(base)),
            NormalPostfixCall::Generic(v) => base = ExpressionBody::GenericCall(v.rebase(base)),
        }
    }
    state.finish(base)
}

impl ThisParser for NormalPostfixCall {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, skip) = ignore(input)?;
        input
            .begin_choice()
            .or_else(|s| ApplyCallNode::parse(s).map_inner(Into::into))
            .or_else(|s| ApplyDotNode::parse(s).map_inner(Into::into))
            .or_else(|s| ViewNode::parse(s).map_inner(Into::into))
            .or_else(|s| GenericCall::parse(s).map_inner(Into::into))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl From<ApplyCallNode<ExpressionBody>> for NormalPostfixCall {
    fn from(value: ApplyCallNode<ExpressionBody>) -> Self {
        NormalPostfixCall::Apply(Box::new(value))
    }
}

impl From<ApplyDotNode<ExpressionBody>> for NormalPostfixCall {
    fn from(value: ApplyDotNode<ExpressionBody>) -> Self {
        NormalPostfixCall::ApplyDot(Box::new(value))
    }
}

impl From<ViewNode<ExpressionBody>> for NormalPostfixCall {
    fn from(value: ViewNode<ExpressionBody>) -> Self {
        NormalPostfixCall::View(Box::new(value))
    }
}
impl From<GenericCall<ExpressionBody>> for NormalPostfixCall {
    fn from(value: GenericCall<ExpressionBody>) -> Self {
        NormalPostfixCall::Generic(Box::new(value))
    }
}

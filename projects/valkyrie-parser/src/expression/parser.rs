use super::*;
use crate::operators::ValkyriePrefix;
use valkyrie_ast::{ApplyCallNode, ApplyTermNode, IdentifierNode};
use valkyrie_types::third_party::pex::{helpers::whitespace, Parsed};

impl ThisParser for PrefixNode<TermExpressionNode> {
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::operator(self.operator.to_string(), &[self.body.as_lisp()])
    }
}
impl ThisParser for InfixNode<TermExpressionNode> {
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::operator(self.operator.to_string(), &[self.lhs.as_lisp(), self.rhs.as_lisp()])
    }
}

impl ThisParser for PostfixNode<TermExpressionNode> {
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::operator(self.operator.to_string(), &[self.body.as_lisp()])
    }
}

impl ThisParser for TermExpressionNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let resolver = ExpressionResolver::default();
        let (state, stream) = ExpressionStream::parse(input)?;
        state.finish(resolver.resolve(stream)?)
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            TermExpressionNode::Placeholder => Lisp::Keyword("placeholder".into()),
            TermExpressionNode::Prefix(v) => v.as_lisp(),
            TermExpressionNode::Binary(v) => v.as_lisp(),
            TermExpressionNode::Suffix(v) => v.as_lisp(),
            TermExpressionNode::Number(v) => v.as_lisp(),
            TermExpressionNode::Symbol(v) => v.as_lisp(),
            TermExpressionNode::String(v) => v.as_lisp(),
            TermExpressionNode::Table(v) => v.as_lisp(),
            TermExpressionNode::Apply(v) => v.as_lisp(),
            TermExpressionNode::ApplyDot(v) => v.as_lisp(),
        }
    }
}

impl ExpressionStream {
    /// term (~ infix ~ term)*
    /// 1 + (1 + +3? + 4)
    pub fn parse(state: ParseState) -> ParseResult<Vec<ExpressionStream>> {
        let mut stream = Vec::with_capacity(4);
        let (state, _) = state.match_fn(|s| parse_term(s, &mut stream))?;
        let (state, _) = state.match_repeats(|s| parse_infix_term(s, &mut stream))?;
        state.finish(stream)
    }
}

/// `~ infix ~ term`
#[inline(always)]
fn parse_infix_term<'i>(input: ParseState<'i>, stream: &mut Vec<ExpressionStream>) -> ParseResult<'i, ()> {
    let (state, infix) = input.skip(ignore).match_fn(ValkyrieInfix::parse).map_inner(ExpressionStream::Infix)?;
    stream.push(infix);
    let (state, _) = state.skip(ignore).match_fn(|s| parse_term(s, stream))?;
    state.finish(())
}

/// `( ~ term ~ )`
pub fn parse_group(input: ParseState) -> ParseResult<Vec<ExpressionStream>> {
    let (state, _) = input.match_char('(')?;
    let (state, group) = state.skip(ignore).match_fn(ExpressionStream::parse)?;
    let (state, _) = state.skip(ignore).match_char(')')?;
    // Only join the global stream after all success
    state.finish(group)
}

/// `(~ prefix)* ~ value (~ suffix)*`
fn parse_term<'i>(state: ParseState<'i>, stream: &mut Vec<ExpressionStream>) -> ParseResult<'i, ()> {
    let (state, _) = state.match_repeats(|s| parse_prefix(s, stream))?;
    let (state, _) = parse_expr_value(state, stream)?;
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
fn parse_expr_value<'a>(input: ParseState<'a>, stream: &mut Vec<ExpressionStream>) -> ParseResult<'a, ()> {
    let (state, term) = input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| parse_group(s).map_inner(ExpressionStream::Group))
        .or_else(|s| parse_value(s).map_inner(ExpressionStream::Term))
        .end_choice()?;

    stream.push(term);
    state.finish(())
}

pub enum NormalPostfixCall {
    Apply(Box<ApplyCallNode<TermExpressionNode>>),
    ApplyDot(Box<ApplyDotNode<TermExpressionNode>>),
}

#[inline]
pub fn parse_value(input: ParseState) -> ParseResult<TermExpressionNode> {
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
            NormalPostfixCall::Apply(v) => base = TermExpressionNode::Apply(v.rebase(base)),
            NormalPostfixCall::ApplyDot(v) => base = TermExpressionNode::ApplyDot(v.rebase(base)),
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
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl From<ApplyCallNode<TermExpressionNode>> for NormalPostfixCall {
    fn from(value: ApplyCallNode<TermExpressionNode>) -> Self {
        NormalPostfixCall::Apply(Box::new(value))
    }
}

impl From<ApplyDotNode<TermExpressionNode>> for NormalPostfixCall {
    fn from(value: ApplyDotNode<TermExpressionNode>) -> Self {
        NormalPostfixCall::ApplyDot(Box::new(value))
    }
}

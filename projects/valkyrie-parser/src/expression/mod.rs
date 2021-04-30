use crate::{helpers::ProgramContext, *};
use nyar_error::{NyarError, Success, Validate, Validation};
use pratt::{Affix, PrattParser, Precedence};
use std::str::FromStr;
use valkyrie_ast::{DotCallNode, GenericCallNode, *};

mod call_dot;
mod call_dot_closure;
mod call_dot_match;
mod call_generic;
mod operators;

impl ExpressionStatementNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionNode> {
        let expr = self.main_expression.build(ctx)?;
        let eos = self.eos.is_some();
        Success { value: ExpressionNode { omit: eos, body: expr, span: self.span.clone() }, diagnostics: vec![] }
    }
}

impl MainExpressionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        let mut stream = vec![];
        let (head, rest) = self.main_term.split_first().expect("at least one term");
        head.push_tokens(&mut stream, ctx)?;
        for (infix, rhs) in self.main_infix.iter().zip(rest.iter()) {
            stream.push(TokenStream::Infix(infix.as_operator()));
            rhs.push_tokens(&mut stream, ctx).valid()?;
        }
        let mut parser = ExpressionResolver;
        let expr = parser.parse(stream.into_iter()).valid()?;
        Success { value: expr, diagnostics: vec![] }
    }
}
impl TypeExpressionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        let mut stream = vec![];
        let (head, rest) = self.type_term.split_first().expect("at least one term");
        head.push_tokens(&mut stream, ctx)?;
        for (infix, rhs) in self.type_infix.iter().zip(rest.iter()) {
            stream.push(TokenStream::Infix(infix.as_operator()));
            rhs.push_tokens(&mut stream, ctx).valid()?;
        }
        let mut parser = ExpressionResolver;
        let expr = parser.parse(stream.into_iter()).valid()?;
        Success { value: expr, diagnostics: vec![] }
    }
}

impl MainTermNode {
    fn push_tokens(&self, stream: &mut Vec<TokenStream>, ctx: &ProgramContext) -> Validation<()> {
        for i in &self.main_prefix {
            stream.push(TokenStream::Prefix(i.as_operator()))
        }
        let main = self.main_factor.build(ctx).valid()?;
        stream.push(TokenStream::Term(main));
        for i in &self.main_suffix_term {
            stream.push(i.as_token(ctx)?)
        }
        Success { value: (), diagnostics: vec![] }
    }
}
impl TypeTermNode {
    fn push_tokens(&self, stream: &mut Vec<TokenStream>, ctx: &ProgramContext) -> Validation<()> {
        for i in &self.type_prefix {
            stream.push(TokenStream::Prefix(i.as_operator()))
        }
        let main = self.main_factor.build(ctx).valid()?;
        stream.push(TokenStream::Term(main));
        for i in &self.type_suffix_term {
            stream.push(i.as_token(ctx)?)
        }
        Success { value: (), diagnostics: vec![] }
    }
}

struct ExpressionResolver;

#[derive(Debug)]
enum TokenStream {
    Prefix(OperatorNode),
    Infix(OperatorNode),
    Term(ExpressionType),
    Postfix(OperatorNode),
    Subscript(SubscriptCallNode),
    Generic(GenericCallNode),
    Apply(ApplyCallNode),
    Dot(DotCallNode),
    DotMatch(MatchCallNode),
    DotClosure(ClosureCallNode),
}

impl<I> PrattParser<I> for ExpressionResolver
where
    I: Iterator<Item = TokenStream>,
{
    type Error = NyarError;
    type Input = TokenStream;
    type Output = ExpressionType;

    fn query(&mut self, input: &Self::Input) -> Result<Affix, Self::Error> {
        let affix = match input {
            TokenStream::Prefix(v) => Affix::Prefix(v.kind.precedence()),
            TokenStream::Infix(v) => Affix::Infix(v.kind.precedence(), v.kind.associativity()),
            TokenStream::Term(_) => Affix::Nilfix,
            TokenStream::Postfix(v) => Affix::Postfix(v.kind.precedence()),
            _ => Affix::Postfix(Precedence(10000)),
        };
        Ok(affix)
    }

    fn primary(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        match input {
            TokenStream::Term(v) => Ok(v),
            _ => unreachable!(),
        }
    }

    fn infix(&mut self, lhs: Self::Output, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error> {
        match op {
            TokenStream::Infix(v) => Ok(BinaryNode { infix: v, lhs, rhs }.into()),
            _ => unreachable!(),
        }
    }

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error> {
        match op {
            TokenStream::Prefix(v) => Ok(UnaryNode { operator: v, base: rhs }.into()),
            _ => unreachable!(),
        }
    }

    fn postfix(&mut self, lhs: Self::Output, op: Self::Input) -> Result<Self::Output, Self::Error> {
        match op {
            TokenStream::Postfix(v) => Ok(UnaryNode { operator: v, base: lhs }.into()),
            TokenStream::Subscript(call) => Ok(call.with_base(lhs).into()),
            TokenStream::Apply(call) => Ok(call.with_base(lhs).into()),
            TokenStream::Dot(call) => Ok(call.with_base(lhs).into()),
            TokenStream::Generic(call) => Ok(call.with_base(lhs).into()),
            TokenStream::DotMatch(call) => Ok(call.with_base(lhs).into()),
            TokenStream::DotClosure(call) => Ok(call.with_base(lhs).into()),
            _ => unreachable!(),
        }
    }
}

impl MainFactorNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        match self {
            Self::Leading(v) => v.build(ctx),
            Self::GroupFactor(v) => v.main_expression.build(ctx),
            Self::NewStatement(v) => v.build(ctx).map(Into::into),
            Self::ObjectStatement(v) => v.build(ctx).map(Into::into),
            Self::DefineLambda(v) => v.build(ctx).map(Into::into),
            Self::TryStatement(v) => v.build(ctx).map(Into::into),
            Self::MatchExpression(v) => v.build(ctx).map(Into::into),
            Self::SwitchStatement(v) => v.build(ctx).map(Into::into),
        }
    }
}
impl TypeFactorNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        match self {
            Self::Leading(v) => v.build(ctx),
            Self::TypeFactor0(_) => {
                todo!()
            }
        }
    }
}

impl MainSuffixTermNode {
    fn as_token(&self, ctx: &ProgramContext) -> Validation<TokenStream> {
        let token = match self {
            Self::InlineSuffixTerm(v) => v.as_token(ctx)?,
            Self::DotMatchCall(v) => TokenStream::DotMatch(v.build(ctx)?),
            Self::DotClosureCall(v) => TokenStream::DotClosure(v.build(ctx)?),
            Self::TupleCall(v) => TokenStream::Apply(v.build(ctx)?),
        };
        Success { value: token, diagnostics: vec![] }
    }
}

impl InlineSuffixTermNode {
    fn as_token(&self, ctx: &ProgramContext) -> Validation<TokenStream> {
        let token = match self {
            Self::MainSuffix(v) => TokenStream::Postfix(v.as_operator()),
            Self::RangeCall(v) => TokenStream::Subscript(v.build(ctx)?),
            Self::InlineTupleCall(v) => TokenStream::Apply(v.build(ctx)?),
            Self::DotCall(v) => TokenStream::Dot(v.build(ctx)?),
            Self::GenericCall(v) => TokenStream::Generic(v.build(ctx)?),
        };
        Success { value: token, diagnostics: vec![] }
    }
}

impl TypeSuffixTermNode {
    fn as_token(&self, ctx: &ProgramContext) -> Validation<TokenStream> {
        let token = match self {
            TypeSuffixTermNode::GenericHide(v) => TokenStream::Generic(v.build(ctx)?),
            TypeSuffixTermNode::TypeSuffix(v) => TokenStream::Postfix(v.as_operator()),
        };
        Success { value: token, diagnostics: vec![] }
    }
}

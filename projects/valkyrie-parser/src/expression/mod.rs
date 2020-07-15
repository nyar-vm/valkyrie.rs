mod display;
mod parser;

use crate::{
    helpers::{ignore, parse_eos, parse_name_join},
    operators::{ValkyrieInfix, ValkyriePrefix, ValkyrieSuffix},
    traits::ThisParser,
};
use lispify::{Lisp, Lispify};
use pratt::{Affix, PrattError, PrattParser};
use std::fmt::Debug;
use valkyrie_ast::{
    ApplyCallNode, ApplyDotNode, ExpressionContext, ExpressionNode, ExpressionType, GenericCall, InfixNode, NamePathNode,
    NumberLiteralNode, PostfixNode, PrefixNode, StringLiteralNode, TableNode, ValkyrieOperator, ViewNode,
};
use valkyrie_types::third_party::pex::{ParseResult, ParseState, StopBecause};
/// A resolver
#[derive(Default)]
pub struct ExpressionResolver {}

pub(crate) struct TypeLevelExpressionType {
    pub wrapper: ExpressionType,
}

impl ExpressionResolver {
    pub fn resolve(&self, stream: Vec<ExpressionStream>) -> Result<ExpressionType, StopBecause> {
        // println!("stream: {stream:#?}");
        let mut effect = ExpressionResolver {};
        match effect.parse(stream.into_iter()) {
            Ok(o) => Ok(o),
            Err(e) => say_stop_reason(e),
        }
    }
}

fn say_stop_reason<T>(e: PrattError<ExpressionStream, StopBecause>) -> Result<T, StopBecause> {
    match e {
        PrattError::UserError(e) => Err(e),
        PrattError::EmptyInput => Err(StopBecause::ShouldNotBe { message: "EOF", position: 0 }),
        PrattError::UnexpectedNilfix(_) => unreachable!(),
        PrattError::UnexpectedPrefix(_) => unreachable!(),
        PrattError::UnexpectedInfix(_) => unreachable!(),
        PrattError::UnexpectedPostfix(_) => unreachable!(),
    }
}

// a..b
// a..<b
// a..=b
// ..=b
// ..<a
// ..a
// ...
// From this
#[derive(Clone, Debug)]
pub enum ExpressionStream {
    Prefix(ValkyriePrefix),
    Postfix(ValkyrieSuffix),
    Infix(ValkyrieInfix),
    Term(ExpressionType),
    Group(Vec<ExpressionStream>),
}

impl ThisParser for ValkyrieOperator {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Operator(self.to_string())
    }
}

impl<I> PrattParser<I> for ExpressionResolver
where
    I: Iterator<Item = ExpressionStream>,
{
    type Error = StopBecause;
    type Input = ExpressionStream;
    type Output = ExpressionType;

    // Query information about an operator (Affix, Precedence, Associativity)
    fn query(&mut self, tree: &ExpressionStream) -> Result<Affix, StopBecause> {
        let affix = match tree {
            ExpressionStream::Infix(o) => Affix::Infix(o.precedence(), o.associativity()),
            ExpressionStream::Postfix(o) => Affix::Postfix(o.precedence()),
            ExpressionStream::Prefix(o) => Affix::Prefix(o.precedence()),
            ExpressionStream::Group(_) => Affix::Nilfix,
            ExpressionStream::Term(_) => Affix::Nilfix,
        };
        Ok(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: ExpressionStream) -> Result<ExpressionType, StopBecause> {
        match tree {
            ExpressionStream::Term(term) => Ok(term),
            ExpressionStream::Group(group) => match self.parse(&mut group.into_iter()) {
                Ok(o) => Ok(o),
                Err(e) => say_stop_reason(e),
            },
            _ => unreachable!(),
        }
    }

    // Construct a binary infix expression, e.g. 1+1
    fn infix(
        &mut self,
        lhs: ExpressionType,
        tree: ExpressionStream,
        rhs: ExpressionType,
    ) -> Result<ExpressionType, StopBecause> {
        match tree {
            ExpressionStream::Infix(o) => Ok(ExpressionType::binary(o.as_operator(), lhs, rhs)),
            _ => unreachable!(),
        }
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: ExpressionStream, rhs: ExpressionType) -> Result<ExpressionType, StopBecause> {
        match tree {
            ExpressionStream::Prefix(o) => Ok(ExpressionType::prefix(o.as_operator(), rhs)),
            _ => unreachable!(),
        }
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: ExpressionType, tree: ExpressionStream) -> Result<ExpressionType, StopBecause> {
        match tree {
            ExpressionStream::Postfix(o) => Ok(ExpressionType::suffix(o.as_operator(), lhs)),
            _ => unreachable!(),
        }
    }
}

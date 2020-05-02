use crate::{helpers::ignore, infix::ValkyrieInfix, prefix::ValkyriePrefix, suffix::ValkyrieSuffix};
use pex::{ParseResult, ParseState, StopBecause};
use pratt::{Affix, PrattError, PrattParser};
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};
mod display;
mod parser;
use crate::{helpers::parse_value, number::ValkyrieNumber, symbol::ValkyrieNamepath};
use std::fmt::Display;

/// A resolver
#[derive(Default)]
pub struct ExpressionResolver {}

impl ExpressionResolver {
    pub fn resolve(&self, stream: Vec<ExpressionStream>) -> Result<ValkyrieExpression, StopBecause> {
        // println!("stream: {stream:#?}");
        let mut effect = ExpressionResolver {};
        match effect.parse(stream.into_iter()) {
            Ok(o) => Ok(o),
            Err(PrattError::UserError(e)) => Err(e)?,
            Err(PrattError::EmptyInput) => unreachable!(),
            Err(PrattError::UnexpectedNilfix(_)) => unreachable!(),
            Err(PrattError::UnexpectedPrefix(_)) => unreachable!(),
            Err(PrattError::UnexpectedInfix(_)) => unreachable!(),
            Err(PrattError::UnexpectedPostfix(_)) => unreachable!(),
        }
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
#[derive(Debug)]
pub enum ExpressionStream {
    Prefix(ValkyriePrefix),
    Postfix(ValkyrieSuffix),
    Infix(ValkyrieInfix),
    Term(ValkyrieExpression),
    Group(Vec<ExpressionStream>),
}

#[derive(Debug)]
pub enum ValkyrieExpression {
    Prefix(Box<ValkyrieUnary>),
    Binary(Box<ValkyrieBinary>),
    Suffix(Box<ValkyrieUnary>),
    Number(Box<ValkyrieNumber>),
    Symbol(Box<ValkyrieNamepath>),
}

#[derive(Debug)]
pub struct ValkyrieUnary {
    pub operator: ValkyrieOperator,
    pub body: ValkyrieExpression,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub struct ValkyrieBinary {
    pub operator: ValkyrieOperator,
    pub lhs: ValkyrieExpression,
    pub rhs: ValkyrieExpression,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub struct ValkyrieOperator {
    pub kind: ValkyrieOperatorKind,
    pub range: Range<usize>,
}

impl ValkyrieExpression {
    pub fn binary(o: ValkyrieOperator, lhs: ValkyrieExpression, rhs: ValkyrieExpression) -> ValkyrieExpression {
        let mut out = ValkyrieExpression::Binary(Box::new(ValkyrieBinary { operator: o, lhs, rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn prefix(o: ValkyrieOperator, rhs: ValkyrieExpression) -> ValkyrieExpression {
        let mut out = ValkyrieExpression::Prefix(Box::new(ValkyrieUnary { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn suffix(o: ValkyrieOperator, rhs: ValkyrieExpression) -> ValkyrieExpression {
        let mut out = ValkyrieExpression::Suffix(Box::new(ValkyrieUnary { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn get_range(&self) -> Range<usize> {
        match self {
            ValkyrieExpression::Prefix(u) => u.range.clone(),
            ValkyrieExpression::Binary(b) => b.range.clone(),
            ValkyrieExpression::Suffix(u) => u.range.clone(),
            ValkyrieExpression::Number(u) => u.range.clone(),
            ValkyrieExpression::Symbol(u) => u.range.clone(),
        }
    }
    pub fn update_range(&mut self) {
        match self {
            ValkyrieExpression::Prefix(u) => {
                let start = u.operator.range.start;
                let end = u.body.get_range().end;
                u.range = start..end;
            }
            ValkyrieExpression::Binary(b) => {
                let start = b.lhs.get_range().start;
                let end = b.rhs.get_range().end;
                b.range = start..end;
            }
            ValkyrieExpression::Suffix(u) => {
                let start = u.body.get_range().start;
                let end = u.operator.range.end;
                u.range = start..end;
            }
            _ => {}
        }
    }
}

impl ValkyrieOperator {
    pub fn new(kind: ValkyrieOperatorKind, range: Range<usize>) -> Self {
        Self { kind, range }
    }
}

#[derive(Debug)]
pub enum ValkyrieOperatorKind {
    /// `!`
    Not,
    /// `+`
    Positive,
    /// `-`
    Negative,
    /// `+`
    Plus,
    /// `++`
    Concat,
    /// `-`
    Minus,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `^`
    Pow,
    /// `==`
    Eq,
    /// `!`
    Unwrap,
    /// `?`
    Raise,
    /// `℃`
    Celsius,
    /// `℉`
    Fahrenheit,
    /// `ᵀ`, `\^T`, `\transpose`
    Transpose,
    /// `ᴴ`, `\^H`, `\conjugate_transpose
    Transjugate,
    Hermitian,
}

impl<I> PrattParser<I> for ExpressionResolver
where
    I: Iterator<Item = ExpressionStream>,
{
    type Error = StopBecause;
    type Input = ExpressionStream;
    type Output = ValkyrieExpression;

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
    fn primary(&mut self, tree: ExpressionStream) -> Result<ValkyrieExpression, StopBecause> {
        match tree {
            ExpressionStream::Term(term) => Ok(term),
            ExpressionStream::Group(group) => match self.parse(&mut group.into_iter()) {
                Ok(o) => Ok(o),
                Err(PrattError::UserError(e)) => Err(e)?,
                Err(PrattError::EmptyInput) => unreachable!(),
                Err(PrattError::UnexpectedNilfix(_)) => unreachable!(),
                Err(PrattError::UnexpectedPrefix(_)) => unreachable!(),
                Err(PrattError::UnexpectedInfix(_)) => unreachable!(),
                Err(PrattError::UnexpectedPostfix(_)) => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    // Construct a binary infix expression, e.g. 1+1
    fn infix(
        &mut self,
        lhs: ValkyrieExpression,
        tree: ExpressionStream,
        rhs: ValkyrieExpression,
    ) -> Result<ValkyrieExpression, StopBecause> {
        match tree {
            ExpressionStream::Infix(o) => Ok(ValkyrieExpression::binary(o.as_operator(), lhs, rhs)),
            _ => unreachable!(),
        }
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: ExpressionStream, rhs: ValkyrieExpression) -> Result<ValkyrieExpression, StopBecause> {
        match tree {
            ExpressionStream::Prefix(o) => Ok(ValkyrieExpression::prefix(o.as_operator(), rhs)),
            _ => unreachable!(),
        }
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: ValkyrieExpression, tree: ExpressionStream) -> Result<ValkyrieExpression, StopBecause> {
        match tree {
            ExpressionStream::Postfix(o) => Ok(ValkyrieExpression::suffix(o.as_operator(), lhs)),
            _ => unreachable!(),
        }
    }
}
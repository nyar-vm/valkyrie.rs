use crate::{helpers::ignore, infix::ValkyrieInfix, prefix::ValkyriePrefix, suffix::ValkyrieSuffix};
use pex::{ParseResult, ParseState, StopBecause};
use pratt::{Affix, PrattParser};
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};
mod display;
mod parser;
use crate::{helpers::parse_value, number::ValkyrieNumber, symbol::ValkyrieNamepath};
use std::fmt::Display;

pub struct ExpressionResolver;

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

impl ValkyrieExpression {
    pub fn binary(o: ValkyrieOperator, lhs: ValkyrieExpression, rhs: ValkyrieExpression) -> ValkyrieExpression {
        ValkyrieExpression::Binary(Box::new(ValkyrieBinary { operator: o, lhs, rhs, range: Default::default() }))
    }
    pub fn unary(o: ValkyrieOperator, rhs: ValkyrieExpression) -> ValkyrieExpression {
        ValkyrieExpression::Prefix(Box::new(ValkyrieUnary { operator: o, rhs, range: Default::default() }))
    }
    pub fn update_range(&mut self) {
        match self {
            ValkyrieExpression::Prefix(u) => {
                todo!()
            }
            ValkyrieExpression::Binary(b) => {
                todo!()
            }
            ValkyrieExpression::Suffix(u) => {
                todo!()
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct ValkyrieUnary {
    operator: ValkyrieOperator,
    rhs: ValkyrieExpression,
    range: Range<usize>,
}

#[derive(Debug)]
pub struct ValkyrieBinary {
    operator: ValkyrieOperator,
    lhs: ValkyrieExpression,
    rhs: ValkyrieExpression,
    range: Range<usize>,
}

#[derive(Debug)]
pub enum ValkyrieOperator {
    /// `!`
    Not,
    /// `+`
    Positive,
    /// `-`
    Negative,
    /// `+`
    Plus,
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
            _ => unreachable!(),
        };
        Ok(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: ExpressionStream) -> Result<ValkyrieExpression, StopBecause> {
        let expr = match tree {
            ExpressionStream::Term(num) => num,
            ExpressionStream::Group(group) => self.parse(&mut group.into_iter()).unwrap(),
            _ => unreachable!(),
        };
        Ok(expr)
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
            ExpressionStream::Prefix(o) => Ok(ValkyrieExpression::unary(o.as_operator(), rhs)),
            _ => unreachable!(),
        }
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: ValkyrieExpression, tree: ExpressionStream) -> Result<ValkyrieExpression, StopBecause> {
        match tree {
            ExpressionStream::Postfix(o) => Ok(ValkyrieExpression::unary(o.as_operator(), lhs)),
            _ => unreachable!(),
        }
    }
}

#[test]
fn main() {
    let tt = ExpressionStream::parse(ParseState::new("1 + 2? ^ 3 ^ 4 + 5 * 6! * 7")).unwrap();
    let expr = ExpressionResolver.parse(&mut tt.into_iter()).unwrap();
    println!("{:#?}", expr);
}

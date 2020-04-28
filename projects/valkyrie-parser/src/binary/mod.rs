use crate::{infix::ValkyrieInfix, prefix::ValkyriePrefix, suffix::ValkyrieSuffix};
use pex::StopBecause;
use pratt::{Affix, Associativity, PrattParser, Precedence};
use regex::Regex;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    str::FromStr,
    sync::LazyLock,
};
mod display;
mod parser;
use crate::number::ValkyrieNumber;
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
            ValkyrieExpression::Number(_) => {}
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
    let tt = vec![
        ExpressionStream::Term(ValkyrieExpression::Number(Box::new(ValkyrieNumber::from_str("1").unwrap()))),
        ExpressionStream::Infix(ValkyrieInfix::from_str("+").unwrap()),
        ExpressionStream::Term(ValkyrieExpression::Number(Box::new(ValkyrieNumber::from_str("2").unwrap()))),
        ExpressionStream::Postfix(ValkyrieSuffix::from_str("?").unwrap()),
    ];
    let expr = ExpressionResolver.parse(&mut tt.into_iter()).unwrap();
    println!("{:#?}", expr);
}

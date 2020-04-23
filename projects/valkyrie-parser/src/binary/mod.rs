use crate::infix::ValkyrieInfix;
use pex::StopBecause;
use pratt::{Affix, Associativity, PrattParser, Precedence};
use regex::Regex;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    str::FromStr,
    sync::LazyLock,
};

pub struct ExpressionResolver;

// a..b
// a..<b
// a..=b
// ..=b
// ..<a
// ..a
// ...
static Prefix: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      [.]{2,3}
    # Temperature
    | [℃℉]
    # Percents
    | [%‰‱]
    # Transpose,adjoint, conjugate
    | [ᵀᴴ]
)"#,
    )
    .unwrap()
});

static SUFFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      [!?]
    # Temperature
    | [℃℉]
    # Percents
    | [%‰‱]
    # Transpose,adjoint, conjugate
    | [ᵀᴴ]
)"#,
    )
    .unwrap()
});

// From this
#[derive(Debug)]
pub enum TokenTree {
    Prefix(char),
    Postfix(char),
    Infix(ValkyrieInfix),
    Primary(i32),
    Group(Vec<TokenTree>),
}

#[derive(Debug)]
pub enum ValkyrieExpression {
    Binary(Box<ValkyrieBinary>),
    Unary(Box<ValkyrieUnary>),
    Primary(i32),
}

impl ValkyrieExpression {
    pub fn binary(o: ValkyrieOperator, lhs: ValkyrieExpression, rhs: ValkyrieExpression) -> ValkyrieExpression {
        ValkyrieExpression::Binary(Box::new(ValkyrieBinary { operator: o, lhs, rhs, range: Default::default() }))
    }
    pub fn unary(o: ValkyrieOperator, rhs: ValkyrieExpression) -> ValkyrieExpression {
        ValkyrieExpression::Unary(Box::new(ValkyrieUnary { operator: o, rhs, range: Default::default() }))
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
    Add,
    // +
    Sub,
    // -
    Mul,
    // *
    Div,
    // /
    Pow,
    // ^
    Eq,
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

#[derive(Debug)]
pub enum UnOpKind {
    Not,
    // !
    Neg,
    // -
    Try, // ?
}

impl<I> PrattParser<I> for ExpressionResolver
where
    I: Iterator<Item = TokenTree>,
{
    type Error = StopBecause;
    type Input = TokenTree;
    type Output = ValkyrieExpression;

    // Query information about an operator (Affix, Precedence, Associativity)
    fn query(&mut self, tree: &TokenTree) -> Result<Affix, StopBecause> {
        let affix = match tree {
            TokenTree::Infix(o) => Affix::Infix(o.precedence(), o.associativity()),
            TokenTree::Postfix('?') => Affix::Postfix(Precedence(5)),
            TokenTree::Prefix('-') => Affix::Prefix(Precedence(6)),
            TokenTree::Prefix('!') => Affix::Prefix(Precedence(6)),
            TokenTree::Group(_) => Affix::Nilfix,
            TokenTree::Primary(_) => Affix::Nilfix,
            _ => unreachable!(),
        };
        Ok(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: TokenTree) -> Result<ValkyrieExpression, StopBecause> {
        let expr = match tree {
            TokenTree::Primary(num) => ValkyrieExpression::Primary(num),
            TokenTree::Group(group) => self.parse(&mut group.into_iter()).unwrap(),
            _ => unreachable!(),
        };
        Ok(expr)
    }

    // Construct a binary infix expression, e.g. 1+1
    fn infix(
        &mut self,
        lhs: ValkyrieExpression,
        tree: TokenTree,
        rhs: ValkyrieExpression,
    ) -> Result<ValkyrieExpression, StopBecause> {
        let op = match tree {
            TokenTree::Infix(o) => o.as_operator(),
            _ => unreachable!(),
        };
        Ok(ValkyrieExpression::binary(op, lhs, rhs))
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: TokenTree, rhs: ValkyrieExpression) -> Result<ValkyrieExpression, StopBecause> {
        let op = match tree {
            TokenTree::Prefix('!') => UnOpKind::Not,
            TokenTree::Prefix('-') => UnOpKind::Neg,
            _ => unreachable!(),
        };
        todo!()
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: ValkyrieExpression, tree: TokenTree) -> Result<ValkyrieExpression, StopBecause> {
        let op = match tree {
            TokenTree::Postfix('?') => UnOpKind::Try,
            _ => unreachable!(),
        };
        todo!()
    }
}

#[test]
fn main() {
    let tt = vec![TokenTree::Primary(1), TokenTree::Infix(ValkyrieInfix::from_str("+").unwrap()), TokenTree::Primary(1)];
    let expr = ExpressionResolver.parse(&mut tt.into_iter()).unwrap();
    println!("Expression: {:#?}", expr);
}

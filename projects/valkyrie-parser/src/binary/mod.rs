use pex::StopBecause;
use pratt::{Affix, Associativity, PrattParser, Precedence};
use regex::Regex;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    sync::LazyLock,
};

pub struct ExpressionResolver;

static BINARY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)
      \#
    | [.]{2}[=<]
    | [.]{1,3}
    | [{}\[\]()]
    | [,;$§¶^]
    | @[*!?@]?
    | [!]?
    # start with <, >
    | >{1,3} | >= | /> | ≥ | ⩾ | ≫
    | <{1,3} | <= | </ | ≤ | ⩽ | <: | <! 
    # start with :
    | ∷ | :: | :> | := | ≔ | :
    # start with -
    | -= | -> | ⟶ | -{1,2}
    # start with ~
    | ~> | ~
    # start with +
    | [+]= | [+]> | [+]{1,2}
    # start with *
    | [*]=?
    # start with / or % or ÷
    | /=?
    | ÷=?
    | %=?
    # start with &
    | &> | &{1,2} | ≻
    | [|]> | [|]{1,2} | ⊁
    | ⊻=? | ⊼=? | ⊽=? | [⩕⩖]
    # start with !
    | != | ≠ | !
    # start with ?
    | [?]{3} | [?]
    # start with =
    | => | ⇒
    | === | == | =
    # unicode
    | [∈∊∉⊑⋢⨳∀∁∂∃∄¬±√∛∜⊹⋗]
    | [⟦⟧⁅⁆⟬⟭]
    | [↻↺⇆↹⇄⇋⇌⇅]
"#,
    )
    .unwrap()
});

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

pub struct ValkyrieInfix {
    normalized: &'static str,
    range: Range<usize>,
}

#[derive(Debug)]
pub enum ValkyrieExpression {
    Binary(Box<ValkyrieBinary>),
    Unary(Box<ValkyrieUnary>),
    Primary(i32),
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

impl Debug for ValkyrieInfix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Infix({})", self.normalized)
    }
}

impl ValkyrieInfix {
    pub const fn new(normalized: &'static str) -> Self {
        Self { normalized, range: Default::default() }
    }
    pub fn precedence(&self) -> Precedence {
        match self.normalized {
            "+" | "-" => Precedence(2),
            "*" | "/" => Precedence(3),
            "^" => Precedence(4),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
    pub fn associativity(&self) -> Associativity {
        match self.normalized {
            "+" | "-" => Associativity::Left,
            "*" | "/" => Associativity::Left,
            "^" => Associativity::Right,
            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
    pub fn as_operator(&self) -> ValkyrieOperator {
        match self.normalized {
            "+" => ValkyrieOperator::Add,
            "-" => ValkyrieOperator::Sub,
            "*" => ValkyrieOperator::Mul,
            "/" => ValkyrieOperator::Div,
            "^" => ValkyrieOperator::Pow,
            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
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
            TokenTree::Primary(num) => ValkyrieExpression::Int(num),
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
            TokenTree::Infix(o) => ValkyrieOperator::Add,
            _ => unreachable!(),
        };
        Ok(Expr::BinOp(ValkyrieBinary {}, ValkyrieInfix {}, Box::new(())))
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: TokenTree, rhs: ValkyrieExpression) -> Result<ValkyrieExpression, StopBecause> {
        let op = match tree {
            TokenTree::Prefix('!') => UnOpKind::Not,
            TokenTree::Prefix('-') => UnOpKind::Neg,
            _ => unreachable!(),
        };
        Ok(ValkyrieExpression::UnOp(op, Box::new(rhs)))
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: ValkyrieExpression, tree: TokenTree) -> Result<ValkyrieExpression, StopBecause> {
        let op = match tree {
            TokenTree::Postfix('?') => UnOpKind::Try,
            _ => unreachable!(),
        };
        Ok(ValkyrieExpression::UnOp(op, Box::new(lhs)))
    }
}

#[test]
fn main() {
    let tt = vec![TokenTree::Primary(1), TokenTree::Infix('+'), TokenTree::Primary(1), TokenTree::Postfix('?')];
    let expr = ExpressionResolver.parse(&mut tt.into_iter()).unwrap();
    println!("Expression: {:#?}", expr);
}

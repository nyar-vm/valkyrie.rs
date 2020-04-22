use pex::StopBecause;
use pratt::{Affix, Associativity, PrattParser, Precedence};
use regex::Regex;
use std::sync::LazyLock;

pub struct ExpressionResolver;

static BINARY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)\\
    | [.]{2}[=<]
    | [.]{1,3}
    | [{}\[\]()]
    | [,;$§¶^]
    | @[*!?@]?
    | \#[!]?
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
    | [℃℉]
    | [↻↺⇆↹⇄⇋⇌⇅]
"#,
    )
    .unwrap()
});

// From this
#[derive(Debug)]
pub enum TokenTree {
    Prefix(char),
    Postfix(char),
    Infix(char),
    Primary(i32),
    Group(Vec<TokenTree>),
}

// To this
#[derive(Debug)]
pub enum Expr {
    BinOp(Box<Expr>, BinOpKind, Box<Expr>),
    UnOp(UnOpKind, Box<Expr>),
    Int(i32),
}

#[derive(Debug)]
pub enum BinOpKind {
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
    Eq, // =
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
    type Output = Expr;

    // Query information about an operator (Affix, Precedence, Associativity)
    fn query(&mut self, tree: &TokenTree) -> Result<Affix, StopBecause> {
        let affix = match tree {
            TokenTree::Infix('=') => Affix::Infix(Precedence(2), Associativity::Neither),
            TokenTree::Infix('+') => Affix::Infix(Precedence(3), Associativity::Left),
            TokenTree::Infix('-') => Affix::Infix(Precedence(3), Associativity::Left),
            TokenTree::Infix('*') => Affix::Infix(Precedence(4), Associativity::Left),
            TokenTree::Infix('/') => Affix::Infix(Precedence(4), Associativity::Left),
            TokenTree::Postfix('?') => Affix::Postfix(Precedence(5)),
            TokenTree::Prefix('-') => Affix::Prefix(Precedence(6)),
            TokenTree::Prefix('!') => Affix::Prefix(Precedence(6)),
            TokenTree::Infix('^') => Affix::Infix(Precedence(7), Associativity::Right),
            TokenTree::Group(_) => Affix::Nilfix,
            TokenTree::Primary(_) => Affix::Nilfix,
            _ => unreachable!(),
        };
        Ok(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: TokenTree) -> Result<Expr, StopBecause> {
        let expr = match tree {
            TokenTree::Primary(num) => Expr::Int(num),
            TokenTree::Group(group) => self.parse(&mut group.into_iter()).unwrap(),
            _ => unreachable!(),
        };
        Ok(expr)
    }

    // Construct a binary infix expression, e.g. 1+1
    fn infix(&mut self, lhs: Expr, tree: TokenTree, rhs: Expr) -> Result<Expr, StopBecause> {
        let op = match tree {
            TokenTree::Infix('+') => BinOpKind::Add,
            TokenTree::Infix('-') => BinOpKind::Sub,
            TokenTree::Infix('*') => BinOpKind::Mul,
            TokenTree::Infix('/') => BinOpKind::Div,
            TokenTree::Infix('^') => BinOpKind::Pow,
            TokenTree::Infix('=') => BinOpKind::Eq,
            _ => unreachable!(),
        };
        Ok(Expr::BinOp(Box::new(lhs), op, Box::new(rhs)))
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: TokenTree, rhs: Expr) -> Result<Expr, StopBecause> {
        let op = match tree {
            TokenTree::Prefix('!') => UnOpKind::Not,
            TokenTree::Prefix('-') => UnOpKind::Neg,
            _ => unreachable!(),
        };
        Ok(Expr::UnOp(op, Box::new(rhs)))
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: Expr, tree: TokenTree) -> Result<Expr, StopBecause> {
        let op = match tree {
            TokenTree::Postfix('?') => UnOpKind::Try,
            _ => unreachable!(),
        };
        Ok(Expr::UnOp(op, Box::new(lhs)))
    }
}

#[test]
fn main() {
    let tt = vec![TokenTree::Primary(1), TokenTree::Infix('+'), TokenTree::Primary(1), TokenTree::Postfix('?')];
    let expr = ExpressionResolver.parse(&mut tt.into_iter()).unwrap();
    println!("Expression: {:#?}", expr);
}

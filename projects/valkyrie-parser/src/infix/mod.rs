use super::*;

mod display;
mod parser;

pub struct ValkyrieInfix {
    pub(crate) normalized: &'static str,
    pub(crate) range: Range<usize>,
}

impl Debug for ValkyrieInfix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Infix({})", self.normalized)
    }
}

impl ValkyrieInfix {
    pub const fn new(normalized: &'static str) -> Self {
        Self { normalized, range: Range { start: 0, end: 0 } }
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

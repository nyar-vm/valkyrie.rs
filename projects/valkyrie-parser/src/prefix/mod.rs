use crate::expression::{ValkyrieOperator, ValkyrieOperatorKind};
use pratt::{Associativity, Precedence};
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};

mod display;
mod parser;

pub struct ValkyriePrefix {
    normalized: String,
    range: Range<usize>,
}

impl Debug for ValkyriePrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Infix({}, {:?})", self.normalized, self.range)
    }
}

impl ValkyriePrefix {
    pub fn new<S: ToString>(s: S, range: Range<usize>) -> ValkyriePrefix {
        ValkyriePrefix { normalized: s.to_string(), range }
    }

    pub fn precedence(&self) -> Precedence {
        match self.normalized.as_str() {
            "+" | "-" => Precedence(2),
            "*" | "/" => Precedence(3),
            "^" => Precedence(4),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
    pub fn associativity(&self) -> Associativity {
        match self.normalized.as_str() {
            "+" | "-" => Associativity::Left,
            "*" | "/" => Associativity::Left,
            "^" => Associativity::Right,
            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
    pub fn as_operator(&self) -> ValkyrieOperator {
        let kind = match self.normalized.as_str() {
            "+" => ValkyrieOperatorKind::Plus,
            "-" => ValkyrieOperatorKind::Minus,
            "*" => ValkyrieOperatorKind::Mul,
            "/" => ValkyrieOperatorKind::Div,
            "^" => ValkyrieOperatorKind::Pow,
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        ValkyrieOperator::new(kind, self.range.clone())
    }
}

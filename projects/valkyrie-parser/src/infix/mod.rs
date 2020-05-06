use crate::expression::{ValkyrieOperator, ValkyrieOperatorKind};
use pratt::{Associativity, Precedence};
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};

mod display;
mod parser;

#[derive(Clone)]
pub struct ValkyrieInfix {
    normalized: String,
    range: Range<usize>,
}

impl ValkyrieInfix {
    pub fn new<S: ToString>(s: S, range: Range<usize>) -> ValkyrieInfix {
        ValkyrieInfix { normalized: s.to_string(), range }
    }

    pub fn precedence(&self) -> Precedence {
        match self.normalized.as_str() {
            "++" => Precedence(100),
            "+" | "-" => Precedence(200),
            "*" | "/" => Precedence(300),
            "^" => Precedence(400),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
    pub fn associativity(&self) -> Associativity {
        match self.normalized.as_str() {
            "++" => Associativity::Left,
            "+" | "-" => Associativity::Left,
            "*" | "/" => Associativity::Left,
            "^" => Associativity::Right,

            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
    pub fn as_operator(&self) -> ValkyrieOperator {
        let kind = match self.normalized.as_str() {
            "++" => ValkyrieOperatorKind::Concat,
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

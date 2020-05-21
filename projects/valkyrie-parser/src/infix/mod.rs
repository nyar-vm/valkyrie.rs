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
    pub fn new<S: AsRef<str>>(infix: S, range: Range<usize>) -> ValkyrieInfix {
        let text = infix.as_ref();
        let mut normalized = String::with_capacity(text.len());
        for c in text.chars() {
            match c {
                ' ' => continue,
                '∈' | '∊' => normalized.push_str("in"),
                '∉' => normalized.push_str("notin"),
                '≫' => normalized.push_str(">>"),
                '≪' => normalized.push_str("<<"),
                '⋙' => normalized.push_str(">>>"),
                '⋘' => normalized.push_str("<<<"),
                '≖' => normalized.push_str("!="),
                _ => normalized.push(c),
            }
        }
        ValkyrieInfix { normalized, range }
    }
    pub fn precedence(&self) -> Precedence {
        match self.normalized.as_str() {
            "++" => Precedence(100),
            "+" | "-" => Precedence(200),
            "*" | "/" => Precedence(300),
            "^" => Precedence(400),
            "==" | "!=" | "<" | ">" => Precedence(100),
            "<<" | ">>" => Precedence(450),
            "<<<" | ">>>" => Precedence(550),
            "in" | "notin" => Precedence(550),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
    pub fn associativity(&self) -> Associativity {
        match self.normalized.as_str() {
            "^" => Associativity::Right,
            _ => Associativity::Left,
        }
    }
    pub fn as_operator(&self) -> ValkyrieOperator {
        let kind = match self.normalized.as_str() {
            "++" => ValkyrieOperatorKind::Concat,
            "+" => ValkyrieOperatorKind::Plus,
            "-" => ValkyrieOperatorKind::Minus,
            "*" => ValkyrieOperatorKind::Multiply,
            "/" => ValkyrieOperatorKind::Divide,
            "^" => ValkyrieOperatorKind::Power,
            ">" => ValkyrieOperatorKind::Greater,
            ">>" => ValkyrieOperatorKind::MuchGreater,
            ">>>" => ValkyrieOperatorKind::VeryMuchGreater,
            "<" => ValkyrieOperatorKind::Less,
            "<<" => ValkyrieOperatorKind::MuchLess,
            "<<<" => ValkyrieOperatorKind::VeryMuchLess,
            "==" => ValkyrieOperatorKind::Equal(true),
            "!=" => ValkyrieOperatorKind::Equal(false),
            "in" => ValkyrieOperatorKind::Belongs(true),
            "notin" => ValkyrieOperatorKind::Belongs(false),

            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        ValkyrieOperator::new(kind, self.range.clone())
    }
}

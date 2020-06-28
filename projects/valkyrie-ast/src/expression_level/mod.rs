pub mod apply;
pub mod ctor;
mod dispatch;
pub mod generic;
pub mod lambda;
pub mod number;
pub mod operators;
pub mod string;
pub mod symbol;
pub mod table;
pub mod view;

use crate::{
    expression_level::table::ArgumentTermNode, utils::comma_terms, ApplyCallNode, ApplyDotNode, ApplyTermNode, GenericCall,
    IdentifierNode, InfixNode, NamePathNode, NumberLiteralNode, OperatorNode, PostfixNode, PrefixNode, StringLiteralNode,
    TableNode, ViewNode,
};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TermExpressionNode {
    pub expression: TermExpressionType,
    pub eos: bool,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TermExpressionType {
    Placeholder,
    Symbol(Box<NamePathNode>),
    Number(Box<NumberLiteralNode>),
    String(Box<StringLiteralNode>),
    Prefix(Box<PrefixNode<Self>>),
    Binary(Box<InfixNode<Self>>),
    Suffix(Box<PostfixNode<Self>>),
    Table(Box<TableNode<Self>>),
    Apply(Box<ApplyCallNode<Self>>),
    ApplyDot(Box<ApplyDotNode<Self>>),
    View(Box<ViewNode<Self>>),
    GenericCall(Box<GenericCall<Self>>),
}

impl TermExpressionType {
    pub fn binary(o: OperatorNode, lhs: TermExpressionType, rhs: TermExpressionType) -> TermExpressionType {
        let mut out = TermExpressionType::Binary(Box::new(InfixNode { operator: o, lhs, rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn prefix(o: OperatorNode, rhs: TermExpressionType) -> TermExpressionType {
        let mut out = TermExpressionType::Prefix(Box::new(PrefixNode { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn suffix(o: OperatorNode, rhs: TermExpressionType) -> TermExpressionType {
        let mut out = TermExpressionType::Suffix(Box::new(PostfixNode { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn get_range(&self) -> Range<usize> {
        match self {
            TermExpressionType::Placeholder => unreachable!("Placeholder expressions should not be called"),
            TermExpressionType::Prefix(u) => u.range.clone(),
            TermExpressionType::Binary(b) => b.range.clone(),
            TermExpressionType::Suffix(u) => u.range.clone(),
            TermExpressionType::Number(u) => u.range.clone(),
            TermExpressionType::Symbol(u) => u.span.clone(),
            TermExpressionType::String(u) => u.span.clone(),
            TermExpressionType::Table(u) => u.range.clone(),
            TermExpressionType::Apply(v) => v.range.clone(),
            TermExpressionType::ApplyDot(v) => v.range.clone(),
            TermExpressionType::View(v) => v.range.clone(),
            TermExpressionType::GenericCall(v) => v.range.clone(),
        }
    }
}

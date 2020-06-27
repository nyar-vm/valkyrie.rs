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
    expression_level::table::ArgumentTermNode, ApplyCallNode, ApplyDotNode, ApplyTermNode, GenericCall, IdentifierNode,
    InfixNode, NamePathNode, NumberLiteralNode, OperatorNode, PostfixNode, PrefixNode, StringLiteralNode, TableNode, ViewNode,
};
use std::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TermExpressionNode {
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

impl TermExpressionNode {
    pub fn binary(o: OperatorNode, lhs: TermExpressionNode, rhs: TermExpressionNode) -> TermExpressionNode {
        let mut out = TermExpressionNode::Binary(Box::new(InfixNode { operator: o, lhs, rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn prefix(o: OperatorNode, rhs: TermExpressionNode) -> TermExpressionNode {
        let mut out = TermExpressionNode::Prefix(Box::new(PrefixNode { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn suffix(o: OperatorNode, rhs: TermExpressionNode) -> TermExpressionNode {
        let mut out = TermExpressionNode::Suffix(Box::new(PostfixNode { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn get_range(&self) -> Range<usize> {
        match self {
            TermExpressionNode::Placeholder => unreachable!("Placeholder expressions should not be called"),
            TermExpressionNode::Prefix(u) => u.range.clone(),
            TermExpressionNode::Binary(b) => b.range.clone(),
            TermExpressionNode::Suffix(u) => u.range.clone(),
            TermExpressionNode::Number(u) => u.range.clone(),
            TermExpressionNode::Symbol(u) => u.span.clone(),
            TermExpressionNode::String(u) => u.span.clone(),
            TermExpressionNode::Table(u) => u.range.clone(),
            TermExpressionNode::Apply(v) => v.range.clone(),
            TermExpressionNode::ApplyDot(v) => v.range.clone(),
            TermExpressionNode::View(v) => v.range.clone(),
            TermExpressionNode::GenericCall(v) => v.range.clone(),
        }
    }
}

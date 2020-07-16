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
    utils::comma_terms, ApplyCallNode, ApplyDotNode, ApplyTermNode, GenericCall, IdentifierNode, InfixNode, NamePathNode,
    NumberLiteralNode, OperatorNode, PostfixNode, PrefixNode, StringLiteralNode, TableNode, ViewNode,
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
pub struct ExpressionNode<T> {
    pub context: ExpressionContext,
    pub expression: T,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExpressionContext {
    Term,
    Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExpressionType {
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

impl<T> ExpressionNode<T> {
    pub fn map<F, U>(self, f: F) -> ExpressionNode<U>
    where
        F: FnOnce(T) -> U,
    {
        ExpressionNode {
            context: self.context,
            expression: f(self.expression),
            range: self.range,
        }
    }
    pub fn with_context(self, context: ExpressionContext) -> ExpressionNode<T> {
        ExpressionNode { context, ..self }
    }
}

impl ExpressionType {
    pub fn binary(o: OperatorNode, lhs: ExpressionType, rhs: ExpressionType) -> ExpressionType {
        let mut out = ExpressionType::Binary(Box::new(InfixNode { operator: o, lhs, rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn prefix(o: OperatorNode, rhs: ExpressionType) -> ExpressionType {
        let mut out = ExpressionType::Prefix(Box::new(PrefixNode { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn suffix(o: OperatorNode, rhs: ExpressionType) -> ExpressionType {
        let mut out = ExpressionType::Suffix(Box::new(PostfixNode { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn get_range(&self) -> Range<usize> {
        match self {
            ExpressionType::Placeholder => unreachable!("Placeholder expressions should not be called"),
            ExpressionType::Prefix(u) => u.range.clone(),
            ExpressionType::Binary(b) => b.range.clone(),
            ExpressionType::Suffix(u) => u.range.clone(),
            ExpressionType::Number(u) => u.range.clone(),
            ExpressionType::Symbol(u) => u.span.clone(),
            ExpressionType::String(u) => u.span.clone(),
            ExpressionType::Table(u) => u.range.clone(),
            ExpressionType::Apply(v) => v.range.clone(),
            ExpressionType::ApplyDot(v) => v.range.clone(),
            ExpressionType::View(v) => v.range.clone(),
            ExpressionType::GenericCall(v) => v.range.clone(),
        }
    }
}

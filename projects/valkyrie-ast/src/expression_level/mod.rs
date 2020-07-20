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

/// The top level elements in script mode.
pub type ExpressionTermNode = ExpressionNode<{ ExpressionContext::Term }>;
/// The top level elements in script mode.
pub type ExpressionTypeNode = ExpressionNode<{ ExpressionContext::Type }>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionNode<const T: ExpressionContext> {
    pub body: ExpressionBody,
    pub range: Range<usize>,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExpressionContext {
    Term,
    Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExpressionBody {
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

impl<const T: ExpressionContext> ExpressionNode<T> {}

impl ExpressionBody {
    pub fn binary(o: OperatorNode, lhs: ExpressionBody, rhs: ExpressionBody) -> ExpressionBody {
        let mut out = ExpressionBody::Binary(Box::new(InfixNode { operator: o, lhs, rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn prefix(o: OperatorNode, rhs: ExpressionBody) -> ExpressionBody {
        let mut out = ExpressionBody::Prefix(Box::new(PrefixNode { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn suffix(o: OperatorNode, rhs: ExpressionBody) -> ExpressionBody {
        let mut out = ExpressionBody::Suffix(Box::new(PostfixNode { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
}

impl ExpressionBody {
    pub fn update_range(&mut self) {}
}

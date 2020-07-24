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
    utils::comma_terms, ApplyCallNode, ApplyDotNode, ApplyTermNode, GenericCall, IdentifierNode, InfixNode, LambdaCallNode,
    LambdaDotNode, NamePathNode, NumberLiteralNode, OperatorNode, PostfixNode, PrefixNode, StatementNode, StringLiteralNode,
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
use indentation::{wrap_display, IndentDisplay, IndentFormatter};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionNode {
    pub r#type: ExpressionType,
    pub body: ExpressionBody,
    pub range: Range<usize>,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExpressionType {
    Term,
    Type,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionContext {
    pub type_level: bool,
    pub allow_newline: bool,
    pub allow_curly: bool,
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
    Apply(Box<ApplyCallNode>),
    ApplyDot(Box<ApplyDotNode>),
    LambdaCall(Box<LambdaCallNode>),
    LambdaDot(Box<LambdaDotNode>),
    View(Box<ViewNode<Self>>),
    GenericCall(Box<GenericCall>),
}

impl Default for ExpressionContext {
    fn default() -> Self {
        ExpressionContext { type_level: false, allow_newline: true, allow_curly: false }
    }
}

impl ExpressionContext {
    pub fn in_type() -> Self {
        ExpressionContext { type_level: true, allow_newline: true, allow_curly: false }
    }
    pub fn in_free() -> Self {
        ExpressionContext { type_level: false, allow_newline: true, allow_curly: true }
    }
    /// In a repl statement
    pub fn in_repl() -> Self {
        ExpressionContext { type_level: false, allow_newline: false, allow_curly: true }
    }
    pub fn as_type(&self) -> ExpressionType {
        if self.type_level { ExpressionType::Type } else { ExpressionType::Term }
    }
}

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

pub mod apply;
pub mod common;
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
    helper::PrettyPrint, ApplyCallNode, ApplyDotNode, CallNode, CallTermNode, GenericCallNode, IdentifierNode, InfixNode,
    LambdaCallNode, LambdaDotNode, NamePathNode, NewConstructNode, NumberLiteralNode, OperatorNode, PostfixNode, PrefixNode,
    PrettyProvider, PrettyTree, StatementNode, StringLiteralNode, SubscriptNode, TableNode,
};
use core::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};
use pretty::{DocAllocator, Pretty};
use std::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

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
    New(Box<NewConstructNode>),
    Prefix(Box<PrefixNode<Self>>),
    Binary(Box<InfixNode<ExpressionNode>>),
    Suffix(Box<PostfixNode<Self>>),
    Table(Box<TableNode>),
    Apply(Box<CallNode<ApplyCallNode>>),
    ApplyDot(Box<ApplyDotNode>),
    LambdaCall(Box<LambdaCallNode>),
    LambdaDot(Box<LambdaDotNode>),
    Subscript(Box<SubscriptNode>),
    GenericCall(Box<CallNode<GenericCallNode>>),
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

impl ExpressionNode {
    pub fn binary(o: OperatorNode, lhs: Self, rhs: Self) -> ExpressionBody {
        let mut out = ExpressionBody::Binary(Box::new(InfixNode { operator: o, lhs, rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn prefix(o: OperatorNode, rhs: Self) -> ExpressionBody {
        let mut out = ExpressionBody::Prefix(Box::new(PrefixNode { operator: o, base: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn suffix(o: OperatorNode, rhs: Self) -> ExpressionBody {
        let mut out = ExpressionBody::Suffix(Box::new(PostfixNode { operator: o, base: rhs, range: Default::default() }));
        out.update_range();
        out
    }
}

impl ExpressionBody {
    pub fn call_generic(base: ExpressionNode, rest: GenericCallNode) -> ExpressionBody {
        let range = base.range.start..rest.range.end;
        let call = CallNode { base, rest, range };
        ExpressionBody::GenericCall(Box::new(call))
    }
    pub fn call_apply(base: ExpressionNode, rest: ApplyCallNode) -> ExpressionBody {
        let range = base.range.start..rest.range.end;
        let call = CallNode { base, rest, range };
        ExpressionBody::Apply(Box::new(call))
    }
}

impl ExpressionBody {
    pub fn update_range(&mut self) {}
}

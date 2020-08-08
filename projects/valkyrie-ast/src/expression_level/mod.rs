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
    pub type_level: bool,
    pub body: ExpressionBody,
    pub range: Range<usize>,
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
    Prefix(Box<PrefixNode>),
    Binary(Box<InfixNode>),
    Suffix(Box<PostfixNode>),
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
    pub fn binary(o: OperatorNode, lhs: Self, rhs: Self) -> Self {
        let ty = lhs.r#type;
        let range = lhs.range.start..rhs.range.end;
        let body = ExpressionBody::Binary(Box::new(InfixNode { operator: o, lhs, rhs }));
        Self { r#type: ty, body, range }
    }
    pub fn prefix(o: OperatorNode, rhs: Self) -> Self {
        let ty = rhs.r#type;
        let range = o.range.start..rhs.range.end;
        let body = ExpressionBody::Prefix(Box::new(PrefixNode { operator: o, base: rhs }));
        Self { r#type: ty, body, range }
    }
    pub fn suffix(o: OperatorNode, lhs: Self) -> Self {
        let ty = lhs.r#type;
        let range = lhs.range.start..o.range.end;
        let body = ExpressionBody::Suffix(Box::new(PostfixNode { operator: o, base: lhs }));
        Self { r#type: ty, body, range }
    }
    pub fn call_generic(base: Self, rest: GenericCallNode) -> Self {
        let range = base.range.start..rest.range.end;

        Self { r#type: base.r#type, body: ExpressionBody::GenericCall(Box::new(CallNode { base, rest })), range }
    }
    pub fn call_apply(base: Self, rest: ApplyCallNode) -> Self {
        let range = base.range.start..rest.range.end;

        Self { r#type: base.r#type, body: ExpressionBody::Apply(Box::new(CallNode { base, rest })), range }
    }
}

impl ExpressionBody {
    pub fn update_range(&mut self) {}
}

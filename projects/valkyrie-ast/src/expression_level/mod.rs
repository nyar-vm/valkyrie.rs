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
pub mod string_template;

use crate::{
    ApplyCallNode, ApplyDotNode, ArgumentTermNode, CallNode, CallTermNode, GenericCallNode, IdentifierNode, InfixNode,
    LambdaCallNode, LambdaDotNode, NamePathNode, NewConstructNode, NumberLiteralNode, OperatorNode, PostfixNode, PrefixNode,
    StatementNode, StringLiteralNode, SubscriptNode, TableNode, TableTermNode,
};
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};
#[cfg(feature = "pretty-print")]
use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionNode {
    pub type_level: bool,
    pub body: ExpressionBody,
    pub span: Range<u32>,
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
    ApplyDot(Box<CallNode<ApplyDotNode>>),
    LambdaCall(Box<CallNode<LambdaCallNode>>),
    LambdaDot(Box<CallNode<LambdaDotNode>>),
    Subscript(Box<CallNode<SubscriptNode>>),
    GenericCall(Box<CallNode<GenericCallNode>>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PostfixCallPart {
    Apply(ApplyCallNode),
    ApplyDot(ApplyDotNode),
    View(SubscriptNode),
    Generic(GenericCallNode),
    Lambda(LambdaCallNode),
    LambdaDot(LambdaDotNode),
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
}

impl ExpressionBody {
    pub fn binary(o: OperatorNode, lhs: Self, rhs: Self) -> Self {
        let span = lhs.span().start..rhs.span().end;
        Self::Binary(Box::new(InfixNode { operator: o, lhs, rhs, span }))
    }
    pub fn prefix(o: OperatorNode, rhs: Self) -> Self {
        let span = o.span.start..rhs.span().end;
        Self::Prefix(Box::new(PrefixNode { operator: o, base: rhs, span }))
    }
    pub fn suffix(o: OperatorNode, lhs: Self) -> Self {
        let span = lhs.span().start..o.span.end;
        Self::Suffix(Box::new(PostfixNode { operator: o, base: lhs, span }))
    }
    pub fn call_generic(base: Self, rest: GenericCallNode) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionBody::GenericCall(Box::new(CallNode { base, rest, span }))
    }
    pub fn call_apply(base: Self, rest: ApplyCallNode) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionBody::Apply(Box::new(CallNode { base, rest, span }))
    }
    pub fn dot_apply(base: Self, rest: ApplyDotNode) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionBody::ApplyDot(Box::new(CallNode { base, rest, span }))
    }
    pub fn call_subscript(base: Self, rest: SubscriptNode) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionBody::Subscript(Box::new(CallNode { base, rest, span }))
    }
    pub fn call_lambda(base: Self, rest: LambdaCallNode) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionBody::LambdaCall(Box::new(CallNode { base, rest, span }))
    }
    pub fn dot_lambda(base: Self, rest: LambdaDotNode) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionBody::LambdaDot(Box::new(CallNode { base, rest, span }))
    }
}

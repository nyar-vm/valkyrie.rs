pub mod annotations;
pub mod apply;
pub mod common;
pub mod ctor;
mod dispatch;
pub mod generic;
pub mod lambda;
pub mod number;
pub mod operators;
pub mod pattern_match;
pub mod string_template;
pub mod symbol;
pub mod table;
pub mod view;
use crate::{
    ApplyCallNode, ApplyDotNode, ArgumentTermNode, CallNode, CallTermNode, CollectsNode, GenericCallNode, IdentifierNode,
    IfLetStatement, IfStatement, InfixNode, LambdaCallNode, LambdaDotNode, LambdaSlotNode, NamePathNode, NewConstructNode,
    NumberLiteralNode, OperatorNode, PatternBranch, PostfixNode, PrefixNode, RaiseNode, StatementNode, StringLiteralNode,
    StringTextNode, SubscriptNode, SwitchStatement, TableNode, TableTermNode,
};
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};
use deriver::From;
#[cfg(feature = "pretty-print")]
use pretty_print::{helpers::PrettySequence, PrettyPrint, PrettyProvider, PrettyTree};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionNode {
    pub type_level: bool,
    pub body: ExpressionType,
    pub span: Range<u32>,
}
/// Temporary node for use in the parser
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TypingExpression {
    pub body: ExpressionType,
    pub span: Range<u32>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionContext {
    pub type_level: bool,
    pub allow_newline: bool,
    pub allow_curly: bool,
}
/// The base expression type
#[derive(Clone, Debug, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExpressionType {
    /// - Placeholder expression
    Placeholder,
    /// - Atomic expression
    Slot(Box<LambdaSlotNode>),
    Symbol(Box<NamePathNode>),
    Number(Box<NumberLiteralNode>),
    Text(Box<StringTextNode>),
    String(Box<StringLiteralNode>),
    New(Box<NewConstructNode>),
    Prefix(Box<PrefixNode>),
    Binary(Box<InfixNode>),
    Suffix(Box<PostfixNode>),
    Table(Box<TableNode>),
    /// - Standalone expression
    Resume(Box<RaiseNode>),
    /// - Standalone expression
    If(Box<IfStatement>),
    /// - Standalone expression
    IfLet(Box<IfLetStatement>),
    /// - Standalone expression
    Switch(Box<SwitchStatement>),
    /// - Postfix expression
    Apply(Box<CallNode<ApplyCallNode>>),
    ApplyDot(Box<CallNode<ApplyDotNode>>),
    LambdaCall(Box<CallNode<LambdaCallNode>>),
    LambdaDot(Box<CallNode<LambdaDotNode>>),
    Subscript(Box<CallNode<SubscriptNode>>),
    GenericCall(Box<CallNode<GenericCallNode>>),
}

/// Temporary node for use in the parser
#[derive(Clone, Debug, PartialEq, Eq, Hash, From)]
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

impl TypingExpression {
    /// Convert this node into a full expression node
    #[allow(clippy::wrong_self_convention)]
    pub fn as_normal(self) -> ExpressionNode {
        ExpressionNode { type_level: true, body: self.body, span: self.span }
    }
}

impl ExpressionType {
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
    pub fn call_generic(base: Self, rest: GenericCallNode, nullable: bool) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionType::GenericCall(Box::new(CallNode { monadic: nullable, base, rest, span }))
    }
    pub fn call_apply(base: Self, rest: ApplyCallNode, nullable: bool) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionType::Apply(Box::new(CallNode { monadic: nullable, base, rest, span }))
    }
    pub fn dot_apply(base: Self, rest: ApplyDotNode, nullable: bool) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionType::ApplyDot(Box::new(CallNode { monadic: nullable, base, rest, span }))
    }
    pub fn call_subscript(base: Self, rest: SubscriptNode, nullable: bool) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionType::Subscript(Box::new(CallNode { monadic: nullable, base, rest, span }))
    }
    pub fn call_lambda(base: Self, rest: LambdaCallNode, nullable: bool) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionType::LambdaCall(Box::new(CallNode { monadic: nullable, base, rest, span }))
    }
    pub fn dot_lambda(base: Self, rest: LambdaDotNode, nullable: bool) -> Self {
        let span = base.span().start..rest.span.end;
        ExpressionType::LambdaDot(Box::new(CallNode { monadic: nullable, base, rest, span }))
    }
}

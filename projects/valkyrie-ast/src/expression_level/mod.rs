pub mod annotations;
pub mod apply;
pub mod common;
pub mod ctor;
mod dispatch;
pub mod generic;
pub mod lambda;
pub mod matches;
pub mod number;
pub mod operators;
pub mod string_template;
pub mod symbol;
pub mod table;
pub mod view;

mod display;

use crate::{
    helper::ValkyrieNode, ApplyCallNode, ApplyDotNode, ArgumentTermNode, ArrayNode, BooleanNode, CallNode, CallTermNode,
    CollectsNode, ExpressionFormatted, GenericCallNode, GuardStatement, IdentifierNode, IfStatement, InfixNode, LambdaCallNode,
    LambdaDotNode, LambdaSlotNode, MatchDotStatement, MonadicDotCall, NamePathNode, NewConstructNode, NullNode,
    NumberLiteralNode, OperatorNode, OutputNode, PatternBlock, PostfixNode, PrefixNode, RaiseNode, StatementNode,
    StringLiteralNode, StringTextNode, SubscriptNode, SwitchStatement, TryStatement, TupleNode, TupleTermNode,
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
    fmt::{Debug, Display, Formatter, Write},
    ops::Range,
};
use deriver::From;
#[cfg(feature = "lispify")]
use lispify::{Lisp, Lispify};
#[cfg(feature = "pretty-print")]
use pretty_print::{helpers::PrettySequence, PrettyPrint, PrettyProvider, PrettyTree};

/// The ast node for an expression
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionRoot {
    /// Weather it is a type level expression
    pub type_level: bool,
    /// The expression body
    pub body: ExpressionNode,
}

/// The ast node for an expression
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionNode {
    /// Weather it is a type level expression
    pub type_level: bool,
    /// The expression body
    pub body: ExpressionType,
    /// The range of the node
    pub span: Range<u32>,
}

/// Temporary node for use in the parser
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TypingExpression {
    /// The type level expression body
    pub body: ExpressionType,
    /// The range of the node
    pub span: Range<u32>,
}

/// Environment for expression parsing
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionContext {
    /// Weather the expression is on type level
    pub type_level: bool,
    /// Weather the expression allow new line in postfix calls
    pub allow_newline: bool,
    /// Weather the expression allow curly braces in postfix calls
    pub allow_curly: bool,
}

/// The base expression type
#[derive(Clone, Debug, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExpressionType {
    /// - Placeholder expression
    Placeholder,
    /// - Atomic expression
    Null(Box<NullNode>),
    /// - Atomic expression
    Boolean(Box<BooleanNode>),
    /// - Atomic expression
    Slot(Box<LambdaSlotNode>),
    /// - Atomic expression
    Symbol(Box<NamePathNode>),
    /// - Atomic expression
    Number(Box<NumberLiteralNode>),
    /// - Atomic expression
    Text(Box<StringTextNode>),
    /// - Atomic expression
    String(Box<StringLiteralNode>),
    /// - Atomic expression
    Formatted(Box<ExpressionFormatted>),
    /// - Atomic expression
    New(Box<NewConstructNode>),
    /// - Compound expression
    Prefix(Box<PrefixNode>),
    /// - Compound expression
    Binary(Box<InfixNode>),
    /// - Compound expression
    Suffix(Box<PostfixNode>),
    /// - Compound expression
    Tuple(Box<TupleNode>),
    /// - Compound expression
    Array(Box<ArrayNode>),
    /// - Standalone expression
    Resume(Box<RaiseNode>),
    /// - Standalone expression
    If(Box<IfStatement>),
    /// - Standalone expression
    IfLet(Box<GuardStatement>),
    /// - Standalone expression
    Switch(Box<SwitchStatement>),
    /// - Standalone expression
    Try(Box<TryStatement>),
    /// - Postfix expression
    Apply(Box<CallNode<ApplyCallNode>>),
    /// - Postfix expression
    ApplyDot(Box<CallNode<ApplyDotNode>>),
    /// - Postfix expression
    LambdaCall(Box<CallNode<LambdaCallNode>>),
    /// - Postfix expression
    LambdaDot(Box<CallNode<LambdaDotNode>>),
    /// - Postfix expression
    Subscript(Box<CallNode<SubscriptNode>>),
    /// - Postfix expression
    GenericCall(Box<CallNode<GenericCallNode>>),
    /// - Postfix expression
    MatchDot(Box<CallNode<MatchDotStatement>>),
    /// - REPL Reference
    OutputReference(Box<OutputNode>),
}

/// Temporary node for resolve postfix calls
#[derive(Clone, PartialEq, Eq, Hash, From)]
pub enum PostfixCallPart {
    /// - Any expression
    Apply(ApplyCallNode),
    /// - Any expression
    ApplyDot(ApplyDotNode),
    /// - Any expression
    View(SubscriptNode),
    /// - Any expression
    Generic(GenericCallNode),
    /// - Standalone expression
    Lambda(LambdaCallNode),
    /// - Standalone expression
    LambdaDot(LambdaDotNode),
    /// - Standalone expression
    Match(MatchDotStatement),
}

impl Default for ExpressionContext {
    fn default() -> Self {
        ExpressionContext { type_level: false, allow_newline: true, allow_curly: false }
    }
}

impl ExpressionContext {
    /// A type level expression
    pub fn in_type() -> Self {
        ExpressionContext { type_level: true, allow_newline: true, allow_curly: false }
    }
    /// A term level expression
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
    /// Build a new binary expression
    pub fn binary(o: OperatorNode, lhs: Self, rhs: Self) -> Self {
        Self::Binary(Box::new(InfixNode { infix: o, lhs, rhs }))
    }
    /// Build a new prefix expression
    pub fn prefix(o: OperatorNode, rhs: Self) -> Self {
        let span = o.span.start..rhs.get_end();
        Self::Prefix(Box::new(PrefixNode { operator: o, base: rhs, span }))
    }
    /// Build a new suffix expression
    pub fn suffix(o: OperatorNode, lhs: Self) -> Self {
        let span = lhs.get_start()..o.span.end;
        Self::Suffix(Box::new(PostfixNode { operator: o, base: lhs, span }))
    }
    /// Build a new expression with generic call
    pub fn call_generic(base: Self, rest: GenericCallNode) -> Self {
        let span = base.get_start()..rest.span.end;
        ExpressionType::GenericCall(Box::new(CallNode { base, rest, span }))
    }
    /// Build a new expression with apply call
    pub fn call_apply(base: Self, rest: ApplyCallNode) -> Self {
        let span = base.get_start()..rest.span.end;
        ExpressionType::Apply(Box::new(CallNode { base, rest, span }))
    }
    /// Build a new expression with apply dot call
    pub fn dot_apply(base: Self, rest: ApplyDotNode) -> Self {
        let span = base.get_start()..rest.span.end;
        ExpressionType::ApplyDot(Box::new(CallNode { base, rest, span }))
    }
    /// Build a new expression with subscript call
    pub fn call_subscript(base: Self, rest: SubscriptNode) -> Self {
        let span = base.get_start()..rest.span.end;
        ExpressionType::Subscript(Box::new(CallNode { base, rest, span }))
    }
    /// Build a new expression with lambda call
    pub fn call_lambda(base: Self, rest: LambdaCallNode) -> Self {
        let span = base.get_start()..rest.span.end;
        ExpressionType::LambdaCall(Box::new(CallNode { base, rest, span }))
    }
    /// Build a new expression with lambda dot call
    pub fn dot_lambda(base: Self, rest: LambdaDotNode) -> Self {
        let span = base.get_start()..rest.span.end;
        ExpressionType::LambdaDot(Box::new(CallNode { base, rest, span }))
    }
    /// Build a new expression with dot match
    pub fn dot_match(base: Self, rest: MatchDotStatement) -> Self {
        let span = base.get_start()..rest.span.end;
        ExpressionType::MatchDot(Box::new(CallNode { base, rest, span }))
    }
}

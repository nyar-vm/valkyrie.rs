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
pub mod range;
pub mod string_template;
pub mod symbol;
pub mod tuple;

mod display;

use crate::{
    helper::ValkyrieNode, ApplyCallNode, ArgumentTermNode, BinaryNode, BooleanNode, CallNode, CallTermNode, ClosureCallNode,
    CollectsNode, ExpressionFormatted, GenericCallNode, GuardStatement, IdentifierNode, IfStatement, LambdaSlotNode,
    MatchDotStatement, ModifiersNode, MonadicDotCall, NamePathNode, NewConstructNode, NullNode, NumberLiteralNode,
    OperatorNode, OutputNode, PatternBlock, RaiseNode, RangeNode, StatementNode, StringLiteralNode, StringTextNode,
    SubscriptCallNode, SwitchStatement, TryStatement, TupleNode, TupleTermNode, UnaryNode,
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
use num_bigint::BigUint;
use nyar_error::{
    third_party::{Associativity, Precedence},
    FileID, FileSpan,
};
#[cfg(feature = "pretty-print")]
use pretty_print::{
    helpers::{KAndRBracket, PrettySequence},
    PrettyPrint, PrettyProvider, PrettyTree,
};
/// The ast node for an expression
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionNode {
    /// Weather to omit to results
    pub omit: bool,
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
#[derive(Clone, PartialEq, Eq, Hash, From)]
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
    Unary(Box<UnaryNode>),
    /// - Compound expression
    Infix(Box<BinaryNode>),
    /// - Compound expression
    Tuple(Box<TupleNode>),
    /// - Compound expression
    Array(Box<RangeNode>),
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
    ApplyCall(Box<ApplyCallNode>),
    /// - Postfix expression
    LambdaCall(Box<ClosureCallNode>),
    /// - Postfix expression
    SubscriptCall(Box<SubscriptCallNode>),
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
    View(SubscriptCallNode),
    /// - Any expression
    Generic(GenericCallNode),
    /// - Standalone expression
    Lambda(ClosureCallNode),
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
        ExpressionNode { omit: true, body: self.body, span: self.span }
    }
}

impl ExpressionType {
    /// Build a new binary expression
    pub fn binary(o: OperatorNode, lhs: Self, rhs: Self) -> Self {
        Self::Infix(Box::new(BinaryNode { infix: o, lhs, rhs }))
    }
    /// Build a new prefix expression
    pub fn prefix(o: OperatorNode, rhs: Self) -> Self {
        Self::Unary(Box::new(UnaryNode { operator: o, base: rhs }))
    }
    /// Build a new suffix expression
    pub fn suffix(o: OperatorNode, lhs: Self) -> Self {
        Self::Unary(Box::new(UnaryNode { operator: o, base: lhs }))
    }
    /// Build a new expression with generic call
    pub fn call_generic(base: Self, rest: GenericCallNode) -> Self {
        let span = base.get_start() as u32..rest.span.end;
        ExpressionType::GenericCall(Box::new(CallNode { base, rest, span }))
    }
    /// Build a new expression with dot match
    pub fn dot_match(base: Self, rest: MatchDotStatement) -> Self {
        let span = base.get_start() as u32..rest.span.end;
        ExpressionType::MatchDot(Box::new(CallNode { base, rest, span }))
    }
}

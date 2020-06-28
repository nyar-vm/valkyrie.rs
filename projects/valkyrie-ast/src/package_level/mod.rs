pub mod classes;
mod dispatch;
pub mod namespace;

use crate::{
    expression_level::TermExpressionNode,
    package_level::{classes::ClassDeclarationNode, namespace::NamespaceDeclarationNode},
    IdentifierNode, LoopStatementNode, TermExpressionType,
};
use alloc::{boxed::Box, string::String, vec::Vec};
use core::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};

/// The top level elements in script mode.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TopStatementNode {
    DeclareNamespace(Box<NamespaceDeclarationNode>),
    DeclareClass(Box<ClassDeclarationNode>),
    Expression(Box<TermExpressionNode>),
}

/// The top level elements in REPL mode.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReplStatementNode {
    DeclareClass(Box<ClassDeclarationNode>),
    Loop(Box<LoopStatementNode>),
    Expression(Box<TermExpressionNode>),
}

/// The valid elements in script mode
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FunctionStatementNode {
    Loop(Box<LoopStatementNode>),
    Expression(Box<TermExpressionNode>),
}

impl From<LoopStatementNode> for FunctionStatementNode {
    fn from(value: LoopStatementNode) -> Self {
        Self::Loop(Box::new(value))
    }
}

impl From<LoopStatementNode> for ReplStatementNode {
    fn from(value: LoopStatementNode) -> Self {
        Self::Loop(Box::new(value))
    }
}

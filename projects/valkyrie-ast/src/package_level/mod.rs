pub mod classes;
mod dispatch;
pub mod function;
pub mod import;
pub mod license;
pub mod namespace;
use crate::{
    package_level::{classes::ClassDeclaration, namespace::NamespaceDeclarationNode},
    ApplyArgumentNode, ArgumentTermNode, ExpressionNode, ForLoopNode, FunctionDeclaration, GenericArgumentNode, IdentifierNode,
    ImportStatementNode, NamePathNode, PrettyPrint, PrettyProvider, PrettyTree, StringLiteralNode, WhileLoopNode,
};
use alloc::{
    borrow::Cow,
    boxed::Box,
    fmt::{Display, Formatter, Write},
    string::String,
    vec,
    vec::Vec,
};
use core::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementNode {
    pub r#type: StatementType,
    pub end_semicolon: bool,
    pub span: Range<u32>,
}

/// The top level elements in script mode.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementType {
    Nothing,
    Namespace(Box<NamespaceDeclarationNode>),
    Import(Box<ImportStatementNode>),
    Class(Box<ClassDeclaration>),
    Function(Box<FunctionDeclaration>),
    While(Box<WhileLoopNode>),
    For(Box<ForLoopNode>),
    Expression(Box<ExpressionNode>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementContext {}

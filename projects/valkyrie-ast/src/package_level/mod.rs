pub mod classes;
mod dispatch;
pub mod function;
pub mod import;
pub mod license;
pub mod namespace;
use crate::{
    package_level::{classes::ClassDeclarationNode, namespace::NamespaceDeclarationNode},
    utils::comma_terms,
    ApplyArgumentNode, ArgumentTermNode, ExpressionNode, ForLoopNode, FunctionDeclarationNode, GenericArgumentNode,
    IdentifierNode, ImportStatementNode, NamePathNode, WhileLoopNode,
};
use core::{
    borrow::BorrowMut,
    fmt::{Debug, Display, Formatter, Write},
    ops::Range,
};
use indentation::{IndentDisplay, IndentFormatter};
use std::{boxed::Box, string::String, vec::Vec};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementNode {
    pub r#type: StatementType,
    pub end_semicolon: bool,
    pub range: Range<usize>,
}

/// The top level elements in script mode.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementType {
    Nothing,
    Namespace(Box<NamespaceDeclarationNode>),
    Import(Box<ImportStatementNode>),
    Class(Box<ClassDeclarationNode>),
    Function(Box<FunctionDeclarationNode>),
    While(Box<WhileLoopNode>),
    For(Box<ForLoopNode>),
    Expression(Box<ExpressionNode>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementContext {}

pub mod classes;
mod dispatch;
pub mod documentation;
pub mod function;
pub mod import;
pub mod let_bind;
pub mod license;
pub mod namespace;

use crate::{
    package_level::{classes::ClassDeclaration, namespace::NamespaceDeclarationNode},
    ApplyArgumentNode, ArgumentTermNode, ControlNode, DocumentationNode, ExpressionNode, ForLoopNode, FunctionDeclaration,
    GenericArgumentNode, IdentifierNode, ImportStatementNode, LetBindNode, NamePathNode, WhileLoopNode,
};
use alloc::{borrow::Cow, boxed::Box, string::String, sync::Arc, vec::Vec};
use core::ops::Range;
use pretty_print::KAndRBracket;
#[cfg(feature = "pretty-print")]
use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};

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
    Document(Box<DocumentationNode>),
    Namespace(Box<NamespaceDeclarationNode>),
    Import(Box<ImportStatementNode>),
    Class(Box<ClassDeclaration>),
    While(Box<WhileLoopNode>),
    For(Box<ForLoopNode>),
    LetBind(Box<LetBindNode>),
    Function(Box<FunctionDeclaration>),
    Control(Box<ControlNode>),
    Expression(Box<ExpressionNode>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementContext {}

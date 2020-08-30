pub mod classes;
mod dispatch;
pub mod documentation;
pub mod flags;
pub mod function;
pub mod guarantee;
pub mod import;
pub mod let_bind;
pub mod license;
pub mod namespace;
pub mod try_catch;
pub mod unions;

use crate::{
    control_flow::for_loop::ForLoop,
    package_level::{classes::ClassDeclaration, namespace::NamespaceDeclarationNode},
    ApplyArgumentNode, ArgumentTermNode, ControlNode, DocumentationNode, ExpressionNode, FunctionDeclaration,
    GenericArgumentNode, GuardStatement, IdentifierNode, ImportStatementNode, LetBindNode, NamePathNode, PatternType,
    WhileLoop,
};
use alloc::{borrow::Cow, boxed::Box, string::String, sync::Arc, vec::Vec};
use core::ops::Range;
use flags::{FlagFieldDeclaration, FlagsDeclaration};
use pretty_print::KAndRBracket;
#[cfg(feature = "pretty-print")]
use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementNode {
    pub r#type: StatementBody,
    pub end_semicolon: bool,
    /// The range of the node
    pub span: Range<u32>,
}

/// The top level elements in script mode.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementBody {
    Nothing,
    Document(Box<DocumentationNode>),
    Namespace(Box<NamespaceDeclarationNode>),
    Import(Box<ImportStatementNode>),
    Class(Box<ClassDeclaration>),
    Flags(Box<FlagsDeclaration>),
    FlagsField(Box<FlagFieldDeclaration>),
    While(Box<WhileLoop>),
    For(Box<ForLoop>),
    LetBind(Box<LetBindNode>),
    Function(Box<FunctionDeclaration>),
    Guard(Box<GuardStatement>),
    Control(Box<ControlNode>),
    Expression(Box<ExpressionNode>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementContext {}

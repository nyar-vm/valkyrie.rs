pub mod classes;
mod dispatch;
pub mod documentation;
pub mod enumerates;
pub mod flags;
pub mod function;
pub mod guarantee;
pub mod import;
pub mod let_bind;
pub mod license;
pub mod namespace;
pub mod try_catch;

use crate::{
    control_flow::for_loop::ForLoop,
    package_level::{classes::ClassDeclaration, namespace::NamespaceDeclaration},
    ApplyArgumentNode, ArgumentTermNode, ClassFieldDeclaration, ControlNode, DocumentationNode, EnumerateDeclaration,
    ExpressionNode, FunctionDeclaration, GenericArgumentNode, GuardStatement, IdentifierNode, ImportStatementNode, LetBindNode,
    ModifiersNode, NamePathNode, PatternType, StatementBlock, VariantDeclaration, WhileLoop,
};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::ops::Range;
use deriver::From;
use flags::{FlagsDeclaration, FlagsFieldDeclaration};
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementBody {
    /// Placeholder for when the parser fails to parse a statement.
    Nothing,
    /// The documentation node, must have acceptor underneath.
    Document(Box<DocumentationNode>),
    /// The namespace declaration node.
    Namespace(Box<NamespaceDeclaration>),
    Import(Box<ImportStatementNode>),
    Class(Box<ClassDeclaration>),
    ClassField(Box<ClassFieldDeclaration>),
    Flags(Box<FlagsDeclaration>),
    FlagsField(Box<FlagsFieldDeclaration>),
    Union(Box<EnumerateDeclaration>),
    Variant(Box<VariantDeclaration>),
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

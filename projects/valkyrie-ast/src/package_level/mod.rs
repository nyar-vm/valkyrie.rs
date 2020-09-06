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
pub mod tagged;
pub mod try_catch;
pub mod unions;

use crate::{
    ApplyArgumentNode, ArgumentTermNode, ClassDeclaration, ClassFieldDeclaration, ClassMethodDeclaration, ControlNode,
    DocumentationNode, EnumerateDeclaration, EnumerateFieldDeclaration, ExpressionNode, FlagsDeclaration, ForLoop,
    FunctionDeclaration, GenericArgumentNode, GuardStatement, IdentifierNode, ImportStatement, LetBindNode, ModifiersNode,
    NamePathNode, NamespaceDeclaration, PatternType, StatementBlock, TaggedDeclaration, UnionDeclaration,
    UnionFieldDeclaration, VariantDeclaration, WhileLoop,
};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::ops::Range;
use deriver::From;
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
    /// The import statement node.
    Import(Box<ImportStatement>),
    /// The class declaration node.
    Class(Box<ClassDeclaration>),
    /// The class's declaration node.
    ClassField(Box<ClassFieldDeclaration>),
    /// The class's method declaration node.
    ClassMethod(Box<ClassMethodDeclaration>),
    /// The union declaration node.
    Union(Box<UnionDeclaration>),
    /// The union's field declaration node.
    UnionField(Box<UnionFieldDeclaration>),
    /// The enumerate declaration node.
    Enumerate(Box<EnumerateDeclaration>),
    /// The enumerates field declaration node.
    EnumerateField(Box<EnumerateFieldDeclaration>),
    /// The flags declaration node.
    Flags(Box<FlagsDeclaration>),
    /// The tagged union declaration node.
    Tagged(Box<TaggedDeclaration>),
    /// The tagged union's variant declaration node.
    Variant(Box<VariantDeclaration>),
    /// The function declaration node.
    Function(Box<FunctionDeclaration>),
    /// The while loop statement node.
    While(Box<WhileLoop>),
    /// The for loop statement node.
    For(Box<ForLoop>),
    /// The let bind statement node.
    LetBind(Box<LetBindNode>),
    /// The guard statement node.
    Guard(Box<GuardStatement>),
    /// The apply argument node.
    Control(Box<ControlNode>),
    /// The apply argument node.
    Expression(Box<ExpressionNode>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementContext {}

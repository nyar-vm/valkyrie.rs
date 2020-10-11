pub mod classes;
pub mod documentation;
pub mod enumerates;
pub mod flags;
pub mod function;
pub mod guarantee;
pub mod import;
pub mod let_bind;
pub mod license;
pub mod namespace;
pub mod program;
pub mod statements;
pub mod tagged;
pub mod try_catch;
pub mod unions;

use crate::{ExpressionBody, GuardLetStatement, StringLiteralNode, StringTextNode};

use crate::{
    AnnotationList, AnnotationNode, ApplyArgumentNode, ArgumentTermNode, ClassDeclaration, ClassFieldDeclaration,
    ClassMethodDeclaration, ControlNode, DocumentationNode, EnumerateDeclaration, EnumerateFieldDeclaration, ExpressionNode,
    FlagsDeclaration, ForLoop, FunctionDeclaration, GenericArgumentNode, GuardStatement, IdentifierNode, ImportStatement,
    LetBindNode, ModifiersNode, NamePathNode, NamespaceDeclaration, PatternExpressionNode, StatementBlock, StatementBody,
    StatementNode, TaggedDeclaration, UnionDeclaration, UnionFieldDeclaration, VariantDeclaration, WhileLoop,
};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};
use core::ops::Range;
use deriver::From;

#[cfg(feature = "pretty-print")]
use pretty_print::{
    helpers::{KAndRBracket, PrettySequence},
    PrettyPrint, PrettyProvider, PrettyTree,
};

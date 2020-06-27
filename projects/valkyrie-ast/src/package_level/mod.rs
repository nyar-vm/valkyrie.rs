use crate::{
    package_level::{classes::ClassDeclarationNode, namespace::NamespaceDeclarationNode},
    TermExpressionNode,
};

pub mod classes;
mod dispatch;
pub mod namespace;

/// A node that represents a statement at the package level.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TopStatementNode {
    DeclareNamespace(Box<NamespaceDeclarationNode>),
    DeclareClass(Box<ClassDeclarationNode>),
    Expression(Box<TermExpressionNode>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FunctionStatementNode {
    Expression(Box<TermExpressionNode>),
}

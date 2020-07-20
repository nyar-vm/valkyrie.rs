use crate::{IdentifierNode, NamePathNode, StringLiteralNode};
use alloc::{boxed::Box, string::ToString, vec, vec::Vec};
use core::{
    fmt::{Display, Formatter},
    ops::Range,
};
mod display;

/// `import namespace node`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportStatementNode {
    pub r#type: ImportStatementType,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportStatementType {
    /// `import root as alias`
    Alias(Box<ImportAliasNode>),
    /// `import root {}`
    Group(Box<ImportGroupNode>),
    /// `import "url" {}`
    String(Box<StringLiteralNode>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportTermNode {
    /// `path as alias`
    Alias(Box<ImportAliasNode>),
    /// `path { group }`
    Group(Box<ImportGroupNode>),
}

/// `path { group }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportGroupNode {
    pub path: NamePathNode,
    pub group: Vec<ImportTermNode>,
}

/// `path as alias`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportAliasNode {
    pub path: NamePathNode,
    pub alias: IdentifierNode,
}

impl ImportGroupNode {
    pub fn new(path: NamePathNode, group: Vec<ImportTermNode>) -> Self {
        Self { path, group }
    }
}

impl ImportAliasNode {
    pub fn new(path: NamePathNode, alias: IdentifierNode) -> Self {
        Self { alias, path }
    }
}

pub struct ImportFlattenTerm {
    pub external: Option<StringLiteralNode>,
    pub path: NamePathNode,
    pub alias: Option<IdentifierNode>,
}

impl ImportStatementNode {
    pub fn flatten(&self) -> Vec<ImportFlattenTerm> {
        match &self.r#type {
            ImportStatementType::Alias(node) => {
                vec![ImportFlattenTerm { external: None, path: node.path.clone(), alias: Some(node.alias.clone()) }]
            }
            ImportStatementType::Group(_node) => {
                todo!();
            }
            ImportStatementType::String(_node) => {
                todo!()
            }
        }
    }
}

impl ImportGroupNode {
    fn resolve(&self, _parent: NamePathNode, _all: &mut Vec<ImportFlattenTerm>) {
        todo!();
    }
}

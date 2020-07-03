use crate::{IdentifierNode, NamePathNode, StringLiteralNode};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{
    fmt::{Display, Formatter},
    ops::Range,
};
mod display;

/// `import namespace node`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportStatementNode {
    pub head: ImportStatementType,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportStatementType {
    /// `import {}`
    Nothing,
    /// `import root as alias`
    Alias(Box<ImportAliasNode>),
    /// `import root {}`
    Symbol(Box<ImportGroupNode>),
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
}

impl ImportStatementNode {
    pub fn flatten(&self) -> Vec<ImportFlattenTerm> {
        vec![]
    }
}

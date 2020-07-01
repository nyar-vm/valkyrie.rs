use crate::{IdentifierNode, NamePathNode, StringLiteralNode};
use alloc::{boxed::Box, string::String, vec, vec::Vec};
use core::ops::Range;

/// `import namespace node`
pub struct ImportStatementNode {
    pub head: ImportRootKind,
    pub group: Vec<ImportTermNode>,
    pub range: Range<usize>,
}

pub enum ImportRootKind {
    /// `import {}`
    Nothing,
    /// `import root {}`
    Symbol(Box<NamePathNode>),
    /// `import "url" {}`
    String(Box<StringLiteralNode>),
}

pub enum ImportTermNode {
    /// `path as alias`
    Alias(Box<ImportAliasNode>),
    /// `path { group }`
    Group(Box<ImportGroupNode>),
}

impl From<ImportAliasNode> for ImportTermNode {
    fn from(value: ImportAliasNode) -> Self {
        Self::Alias(Box::new(value))
    }
}

impl From<ImportGroupNode> for ImportTermNode {
    fn from(value: ImportGroupNode) -> Self {
        Self::Group(Box::new(value))
    }
}

/// `path { group }`
pub struct ImportGroupNode {
    pub path: NamePathNode,
    pub group: Vec<ImportTermNode>,
}

/// `path as alias`
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

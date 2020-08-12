mod display;

use super::*;
use alloc::borrow::ToOwned;

/// `import namespace node`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportStatementNode {
    pub path: Option<StringLiteralNode>,
    pub term: ImportTermNode,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportTermNode {
    /// `a::b::c`
    Item(Box<NamePathNode>),
    /// `a::b::c as alias`
    Alias(Box<ImportAliasNode>),
    /// `a::b { c::d }`
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

#[derive(Clone, Default)]
pub struct ImportResolvedItem {
    pub external: Option<StringLiteralNode>,
    pub path: Vec<IdentifierNode>,
    pub alias: Option<IdentifierNode>,
}

impl ImportResolvedItem {
    pub fn join_external(&self, name: &StringLiteralNode) -> Self {
        Self { external: Some(name.clone()), ..self.clone() }
    }
    pub fn join_name(&self, name: &IdentifierNode) -> Self {
        let mut path = self.path.clone();
        path.push(name.clone());
        Self { path, ..self.clone() }
    }
    pub fn join_path(&self, namepath: &[IdentifierNode]) -> Self {
        let mut path = self.path.clone();
        path.extend_from_slice(namepath);
        Self { path, ..self.clone() }
    }
    pub fn join_alias(&self, alias: &IdentifierNode) -> Self {
        Self { alias: Some(alias.clone()), ..self.clone() }
    }
}

impl ImportStatementNode {
    pub fn flatten(&self) -> Vec<ImportResolvedItem> {
        let root = ImportResolvedItem { external: self.path.clone(), path: Vec::new(), alias: None };
        let mut all = Vec::new();
        self.term.resolve(&root, &mut all);
        all
    }
}

impl ImportTermNode {
    fn resolve(&self, parent: &ImportResolvedItem, all: &mut Vec<ImportResolvedItem>) {
        match self {
            ImportTermNode::Item(item) => all.push(parent.join_path(&item.names)),
            ImportTermNode::Alias(alias) => {
                all.push(parent.join_path(&alias.path.names).join_alias(&alias.alias));
            }
            ImportTermNode::Group(group) => {
                let root = parent.join_path(&group.path.names);
                for item in &group.group {
                    item.resolve(&root, all);
                }
            }
        }
    }
}

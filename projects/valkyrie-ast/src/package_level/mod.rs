// use serde::{Deserialize, Serialize};
// use std::{ops::Range, str::FromStr};
//
// use valkyrie_errors::FileID;
//
// use crate::{ValkyrieASTKind, ValkyrieASTNode, ValkyrieIdentifier};
// use std::slice::Iter;
//
// pub mod class_field;
// pub mod class_method;
// pub mod classes;
//
mod arithmetic;

use crate::{utils::small_range, IdentifierNode};
use std::ops::Range;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NamespaceKind {
    // In the v language, there only one shared namespace
    Shared,
    // In the v language, there only one shared namespace
    Unique,
    // In the v language, there only one shared namespace
    Test,
}

#[derive(Clone, Debug, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamespaceDeclareNode {
    pub kind: NamespaceKind,
    pub path: Vec<IdentifierNode>,
    pub span: Range<u32>,
}

impl NamespaceDeclareNode {
    pub fn new<I>(names: I, range: &Range<usize>) -> Self
    where
        I: IntoIterator<Item = IdentifierNode>,
    {
        Self { kind: NamespaceKind::Unique, path: names.into_iter().collect(), span: small_range(range) }
    }
    pub fn with_kind(mut self, kind: NamespaceKind) -> Self {
        self.kind = kind;
        self
    }
}

//
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct NamespaceDeclare {
//     pub kind: NamespaceKind,
//     pub name: Vec<String>,
// }
//
// impl NamespaceDeclare {
//     pub fn new(kind: &str) -> Self {
//         Self { kind: NamespaceKind::from_str(kind).unwrap(), name: Vec::new() }
//     }
//     pub fn push_name(&mut self, name: impl Into<String>) {
//         self.name.push(name.into());
//     }
//     pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
//         ValkyrieASTKind::Namespace(box self).to_node(file, range)
//     }
// }
//
// impl FromStr for NamespaceKind {
//     type Err = !;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let out = match s {
//             "namespace!" => Self::Shared,
//             "namespace*" => Self::Test,
//             _ => Self::Unique,
//         };
//         Ok(out)
//     }
// }

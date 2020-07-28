use crate::{OperatorNode, PrettyPrint, PrettyProvider};
use pretty::{termcolor::ColorSpec, DocAllocator, RefDoc};

use super::*;

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

impl NamespaceKind {
    /// Get the string representation of the namespace kind
    pub fn as_str(&self) -> &'static str {
        match self {
            NamespaceKind::Shared => "namespace",
            NamespaceKind::Unique => "namespace!",
            NamespaceKind::Test => "namespace?",
        }
    }
}

impl PrettyPrint for NamespaceDeclarationNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> RefDoc<'a, ColorSpec> {
        let head = allocator.text(self.kind.as_str()).annotate(allocator.keyword_style());
        let space = allocator.space();
        let path = allocator.intersperse(self.path.iter().map(|id| id.pretty(allocator)), allocator.text("."));
        head.append(space).append(path).into_doc()
    }
}

impl Display for NamespaceDeclarationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.pretty_string(9999))
    }
}

/// `namespace std.math`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamespaceDeclarationNode {
    pub kind: NamespaceKind,
    pub path: Vec<IdentifierNode>,
    pub span: Range<usize>,
}

impl NamespaceDeclarationNode {
    pub fn new<I>(names: I, range: &Range<usize>) -> Self
    where
        I: IntoIterator<Item = IdentifierNode>,
    {
        Self { kind: NamespaceKind::Unique, path: names.into_iter().collect(), span: range.clone() }
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

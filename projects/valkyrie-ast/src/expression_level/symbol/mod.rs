use super::*;

mod display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNode {
    pub name: String,
    /// The range of the node
    pub span: Range<u32>,
}

/// `package∷module∷name`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamePathNode {
    /// The names of the identifier.
    pub names: Vec<IdentifierNode>,
    /// The range of the identifier.
    pub span: Range<u32>,
}

/// `$, $1, $x`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaSlotNode {
    pub name: String,
    pub span: Range<u32>,
}

impl LambdaSlotNode {
    pub fn new<S>(name: S, span: Range<u32>) -> Self
    where
        S: ToString,
    {
        Self { name: name.to_string(), span }
    }
}

/// A namepath is a series of identifiers separated by dots.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MacroKind {
    /// `@`
    Normal,
    /// `@@`
    Environment,
    /// `@!`
    NonCapture,
}

/// `@module∷name.function`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MacroPathNode {
    /// The names of the identifier.
    pub path: NamePathNode,
    /// The names of the identifier.
    pub names: Vec<IdentifierNode>,
    /// The range of the identifier.
    pub span: Range<u32>,
}

impl NamePathNode {
    pub fn new<I>(names: I) -> Self
    where
        I: IntoIterator<Item = IdentifierNode>,
    {
        let names: Vec<IdentifierNode> = names.into_iter().collect();
        let start = names.iter().map(|s| s.span.start).min().unwrap_or(0);
        let end = names.iter().map(|n| n.span.end).max().unwrap_or(0);
        Self { names, span: start..end }
    }
    pub fn join<I: IntoIterator<Item = IdentifierNode>>(mut self, other: I) -> Self {
        self.names.extend(other);
        self
    }
}

impl MacroPathNode {
    pub fn new(path: NamePathNode, names: Vec<IdentifierNode>, span: Range<u32>) -> Self {
        Self { path, names, span }
    }
}

impl IdentifierNode {
    pub fn new<S: ToString>(s: S, span: Range<u32>) -> Self {
        Self { name: s.to_string(), span }
    }
    pub fn is_normal(&self) -> bool {
        self.name.starts_with(|c: char| c.is_ascii_lowercase())
    }
}

// impl Display for ValkyrieIdentifier {
//     fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
//         // if string is pure xid
//         write!(f, "{}", self.name)
//         // if self.name.chars().all(|c| c.is_xid_continue()) {
//         //     write!(f, "{}", self.name)
//         // } else {
//         //     write!(f, "\"{}\"", self.name)
//         // }
//     }
// }
//
// impl ValkyrieIdentifier {
//     pub fn new(name: impl Into<String>, file: FileID, range: &Range<usize>) -> Self {
//         Self { name: name.into(), span: FileSpan { file, head: range.start, tail: range.end } }
//     }
//     pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
//         ValkyrieASTKind::Namepath(vec![self]).to_node(file, range)
//     }
// }
//
// impl ValkyrieASTNode {
//     pub fn identifier(name: impl Into<String>, file: FileID, range: &Range<usize>) -> Self {
//         ValkyrieIdentifier::new(name, file, range).to_node(file, range)
//     }
//     pub fn namepath(items: Vec<ValkyrieIdentifier>, file: FileID, range: &Range<usize>) -> Self {
//         Self { kind: ValkyrieASTKind::Namepath(items), span: FileSpan::new(file, range) }
//     }
// }

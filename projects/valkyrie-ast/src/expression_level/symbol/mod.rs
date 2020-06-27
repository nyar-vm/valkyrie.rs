use super::*;

mod display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNode {
    pub name: String,
    pub span: Range<usize>,
}

/// `package∷module∷name`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamePathNode {
    /// The names of the identifier.
    pub names: Vec<IdentifierNode>,
    /// The range of the identifier.
    pub span: Range<usize>,
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
    Dict,
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
    pub span: Range<usize>,
}

impl NamePathNode {
    pub fn new<I>(names: I, range: Range<usize>) -> Self
    where
        I: IntoIterator<Item = IdentifierNode>,
    {
        Self { names: names.into_iter().collect(), span: range }
    }
}

impl MacroPathNode {
    pub fn new(path: NamePathNode, names: Vec<IdentifierNode>, range: Range<usize>) -> Self {
        Self { path, names, span: range }
    }
}

impl IdentifierNode {
    pub fn new<S: ToString>(s: S, range: Range<usize>) -> Self {
        Self { name: s.to_string(), span: range }
    }
    pub fn is_normal(&self) -> bool {
        self.name.starts_with(|c: char| c.is_ascii_lowercase())
    }
}

// impl Display for ValkyrieIdentifier {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

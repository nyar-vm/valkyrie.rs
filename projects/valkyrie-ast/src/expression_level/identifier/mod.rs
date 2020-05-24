use super::*;
mod display;

#[derive(Clone, Debug, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValkyrieIdentifier {
    pub name: String,
    pub span: Range<u32>,
}

/// A namepath is a series of identifiers separated by dots.
#[derive(Clone, Debug, Eq)]
pub struct ValkyrieNamepath {
    /// The names of the identifier.
    pub names: Vec<ValkyrieIdentifier>,
    /// The range of the identifier.
    pub range: Range<u32>,
}

impl ValkyrieIdentifier {
    pub fn is_normal(&self) -> bool {
        self.name.starts_with(|c: char| c.is_ascii_lowercase())
    }
}

impl ValkyrieIdentifier {
    pub fn new<S: ToString>(s: S, range: &Range<usize>) -> Self {
        Self { name: s.to_string(), span: small_range(range) }
    }
}

impl Display for ValkyrieIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl Display for ValkyrieIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
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

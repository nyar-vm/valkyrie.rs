use super::*;
#[cfg(feature = "pretty-print")]
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
}
/// `package∷module∷name`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BooleanNode {
    pub value: bool,
    /// The range of the node
    pub span: Range<u32>,
}
impl ValkyrieNode for BooleanNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
/// `null, nil`, type of null value
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NullNode {
    /// Whether the collection is empty or does not exist
    pub nil: bool,
    /// The range of the node
    pub span: Range<u32>,
}
impl ValkyrieNode for NullNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
/// `%1, %%1`, the number of the reference
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutputNode {
    /// - positive number indicates the nth one
    /// - negative number indicates the nth one from the last
    pub count: isize,
    /// The range of the node
    pub span: Range<u32>,
}
impl ValkyrieNode for OutputNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
/// `$, $1, $x`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaSlotNode {
    pub name: String,
    pub span: Range<u32>,
}

impl ValkyrieNode for LambdaSlotNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}

impl LambdaSlotNode {
    pub fn new<S>(name: S, span: Range<u32>) -> Self
    where
        S: ToString,
    {
        Self { name: name.to_string(), span }
    }
}

impl NamePathNode {
    /// Create a new name path node with given identifiers.
    pub fn new<I>(names: I) -> Self
    where
        I: IntoIterator<Item = IdentifierNode>,
    {
        Self { names: names.into_iter().collect() }
    }
    pub fn join<I: IntoIterator<Item = IdentifierNode>>(mut self, other: I) -> Self {
        self.names.extend(other);
        self
    }
    /// Calculate range by first and last elements
    pub fn get_range(&self) -> Range<usize> {
        let head = self.names.first().map(|x| x.span.start).unwrap_or_default() as usize;
        let tail = self.names.last().map(|x| x.span.end).unwrap_or_default() as usize;
        head..tail
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

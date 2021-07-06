use super::*;

mod display;

/// A node representing a identifier.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNode {
    /// The name of the identifier.
    pub name: String,
    /// The location of this identifier.
    pub span: FileSpan,
}

/// `package∷module∷name`
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamePathNode {
    /// The names of the identifier.
    pub path: Vec<IdentifierNode>,
    pub span: FileSpan,
}
/// `package∷module∷name`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BooleanNode {
    /// `true, false`
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
    /// The repeat times of `$`
    pub level: usize,
    /// The name of the slot
    pub item: LambdaSlotItem,
    /// The location of the slot
    pub span: Range<u32>,
}

/// `$, $0, $1, $x`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LambdaSlotItem {
    /// `$, $.call()`, Automatically obtain a number based on context
    SelfType,
    /// `$0, type_of($)`, The meta info of the slot
    MetaType,
    /// `$1, $.arguments.1`, The slot index from the lambda
    Index(NonZeroU64),
    /// `$x, $.arguments.x`, The slot name from the lambda
    Named(IdentifierNode),
}

impl ValkyrieNode for LambdaSlotNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}

impl FromIterator<IdentifierNode> for NamePathNode {
    fn from_iter<T: IntoIterator<Item = IdentifierNode>>(iter: T) -> Self {
        Self { path: iter.into_iter().collect(), span: Default::default() }
    }
}

impl NamePathNode {
    /// Create a new name path node with given identifiers.
    pub fn new<I>(names: I) -> Self
    where
        I: IntoIterator<Item = IdentifierNode>,
    {
        Self { path: names.into_iter().collect(), span: Default::default() }
    }
    /// Push a new identifier to the name path.
    pub fn join<I: IntoIterator<Item = IdentifierNode>>(mut self, other: I) -> Self {
        self.path.extend(other);
        self
    }
    /// Calculate range by first and last elements
    pub fn get_range(&self) -> Range<usize> {
        self.span.get_range()
    }
    pub fn with_span(self, span: FileSpan) -> Self {
        Self { span, ..self }
    }
}

impl IdentifierNode {
    /// Create a new identifier node with given name.
    pub fn new<S: ToString>(s: S) -> Self {
        Self { name: s.to_string(), span: Default::default() }
    }
    /// Set the file for namepath
    pub fn with_file(mut self, file: FileID) -> Self {
        self.span.set_file(file);
        self
    }
    /// Set the
    pub fn with_span<I>(mut self, range: Range<usize>) -> Self {
        self.span.set_range(range);
        self
    }
    /// Whether this identifier needs escaping
    pub fn is_normal(&self) -> bool {
        self.name.starts_with(|c: char| c.is_ascii_lowercase())
    }
}

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

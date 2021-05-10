use super::*;
use crate::helper::WrapDisplay;

mod display;

/// The annotations of the statements
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationNode {
    /// The documentations of the statement
    pub documents: DocumentationNode,
    /// The attributes of the statement
    pub attributes: AnnotationList,
    /// The modifiers of the statement
    pub modifiers: ModifierList,
}

/// A namepath is a series of identifiers separated by dots.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeKind {
    /// `@`
    Normal,
    /// `@@`
    Environment,
    /// `@!`
    NonCapture,
}

impl Default for AttributeKind {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for AnnotationNode {
    fn default() -> Self {
        Self { documents: Default::default(), attributes: Default::default(), modifiers: Default::default() }
    }
}

/// `@module∷name.variant(args) <CAPTURE>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeNode {
    /// The kind of the attributes
    pub kind: AttributeKind,
    /// The arguments of the attributes
    pub term: AttributeTerm,
    /// The range of the node
    pub span: Range<u32>,
}

/// `@[module∷name.function(args), module∷name.function2(args)] <CAPTURE>`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationList {
    pub kind: AttributeKind,
    pub terms: Vec<AttributeTerm>,
    /// The location of the annotation
    pub span: Range<u32>,
}

/// `module∷name.variant(args) <CAPTURE>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeTerm {
    pub path: AnnotationPathNode,
    pub arguments: ApplyCallNode,
    pub collects: CollectorNode,
}

/// `@{ module∷name.function(args) <CAPTURE>, module∷name.function2(args) <CAPTURE> }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationStatements {
    pub kind: AttributeKind,
    pub terms: Vec<AnnotationPathNode>,
}

/// `@module∷name.function`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationPathNode {
    /// The names of the identifier.
    pub path: NamePathNode,
    /// The names of the identifier.
    pub names: Vec<IdentifierNode>,
    /// The range of the identifier.
    pub span: Range<u32>,
}

/// `public static final synchronized class Main {}`
///
/// - Auxiliary parsing function, not instantiable.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifierList {
    /// The modifiers in group
    pub terms: Vec<IdentifierNode>,
}

impl From<AttributeNode> for AnnotationList {
    fn from(value: AttributeNode) -> Self {
        Self { kind: value.kind, terms: vec![value.term], span: value.span }
    }
}

impl AnnotationNode {
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty() && self.attributes.is_empty() && self.modifiers.is_empty()
    }
}

impl AttributeKind {
    /// Returns the string representation of the macro kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "@",
            Self::Environment => "@@",
            Self::NonCapture => "@!",
        }
    }
}

impl AnnotationPathNode {
    pub fn new(path: NamePathNode, names: Vec<IdentifierNode>, span: Range<u32>) -> Self {
        Self { path, names, span }
    }
}

impl AttributeTerm {
    /// Expand to the standard annotation form.
    pub fn expand(self) -> AnnotationStatements {
        todo!()
    }
}

impl AnnotationList {
    /// Check if the modifier is present.
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }
    /// Expand to the standard annotation form.
    pub fn expand(self) -> AnnotationStatements {
        todo!()
    }
}

impl ModifierList {
    /// Check if the modifier is present.
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    /// Check if the modifier is present.
    pub fn contains(&self, modifier: &str) -> bool {
        self.terms.iter().any(|x| x.name.eq(modifier))
    }
}

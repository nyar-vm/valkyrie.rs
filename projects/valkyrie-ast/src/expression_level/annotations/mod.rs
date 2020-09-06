use super::*;
#[cfg(feature = "pretty-print")]
mod display;

/// A namepath is a series of identifiers separated by dots.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnnotationKind {
    /// `@`
    Normal,
    /// `@@`
    Environment,
    /// `@!`
    NonCapture,
}

/// `@module∷name.variant(args) <CAPTURE>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationNode {
    pub kind: AnnotationKind,
    pub term: AnnotationTerm,
    pub span: Range<u32>,
}

/// `@[module∷name.function(args), module∷name.function2(args)] <CAPTURE>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationList {
    pub kind: AnnotationKind,
    pub terms: Vec<AnnotationTerm>,
    pub span: Range<u32>,
}

/// `module∷name.variant(args) <CAPTURE>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationTerm {
    pub path: AnnotationPathNode,
    pub arguments: ApplyCallNode,
    pub collects: CollectsNode,
}

/// `@{ module∷name.function(args) <CAPTURE>, module∷name.function2(args) <CAPTURE> }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationStatements {
    pub kind: AnnotationKind,
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
pub struct ModifiersNode {
    pub terms: Vec<IdentifierNode>,
}

impl From<AnnotationNode> for AnnotationList {
    fn from(value: AnnotationNode) -> Self {
        Self { kind: value.kind, terms: vec![value.term], span: value.span }
    }
}

impl AnnotationKind {
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

impl AnnotationTerm {
    /// Expand to the standard annotation form.
    pub fn expand(self) -> AnnotationStatements {
        todo!()
    }
}

impl AnnotationList {
    /// Expand to the standard annotation form.
    pub fn expand(self) -> AnnotationStatements {
        todo!()
    }
}

impl ModifiersNode {
    pub fn new(modifiers: Vec<IdentifierNode>) -> Self {
        Self { terms: modifiers }
    }
    pub fn contains(&self, modifier: &str) -> bool {
        self.terms.iter().any(|x| x.name.eq(modifier))
    }
}

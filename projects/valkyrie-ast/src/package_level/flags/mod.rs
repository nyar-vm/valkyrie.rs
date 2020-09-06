use super::*;

mod display;
mod iters;

/// `flags Bit(8bits): Trait { FlagA, FlagB }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlagsDeclaration {
    /// The documentation for this flag.
    pub documentation: DocumentationNode,
    /// `flags Name`
    pub namepath: NamePathNode,
    /// The modifiers for this flag.
    pub modifiers: Vec<IdentifierNode>,
    /// `(8bits)`
    pub layout: Option<ExpressionNode>,
    /// `: Trait`
    pub implements: Vec<String>,
    /// `{ FlagA, FlagB }`
    pub body: StatementBlock,
    /// The range of the node.
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FlagsTerm {
    Field(EnumerateFieldDeclaration),
}

#[derive(Clone, Debug)]
pub struct FlagsIterator<'a> {
    inner: core::slice::Iter<'a, StatementNode>,
}

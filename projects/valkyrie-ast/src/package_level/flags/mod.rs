use super::*;
mod display;
mod iters;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FlagsKind {
    /// aka. `enumerate`,
    Exclusive,
    /// aka. `flags`
    Juxtapose,
}

/// a number that encodes special semantics
///
/// `enumerate Bit(8bits): Trait { FlagA, FlagB }`
///
/// `flags Bit(8bits): Trait { FlagA, FlagB }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlagDeclaration {
    /// The documentation for this flag.
    pub documentation: DocumentationNode,
    /// The kind of the flag statement
    pub kind: FlagsKind,
    /// The name of the flag.
    pub name: IdentifierNode,
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

/// `Name = 0x00`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlagFieldDeclaration {
    /// The documentation for this field.
    pub documentation: DocumentationNode,
    /// The identifier of the field.
    pub name: IdentifierNode,
    /// The value of the field if exists.
    pub value: Option<ExpressionNode>,
    /// The range of the node.
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FlagTerm {
    Field(FlagFieldDeclaration),
}

#[derive(Clone, Debug)]
pub struct FlagIterator<'a> {
    inner: core::slice::Iter<'a, StatementNode>,
}

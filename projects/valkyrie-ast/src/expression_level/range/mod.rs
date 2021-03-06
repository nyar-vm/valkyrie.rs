use super::*;

mod display;

/// `[index, start:end:step]`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RangeNode {
    /// The raw string of the number.
    pub terms: Vec<RangeTermNode>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// `a: item`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RangeTermNode {
    Index(),
}

/// The key of tuple
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TupleKeyType {
    /// This expression has no key
    Nothing,
    /// A valid identifier key, or a string key
    Identifier(IdentifierNode),
    /// A valid number key
    Number(BigUint),
    /// A subscript key
    Subscript(SubscriptCallNode),
}

impl Default for TupleKeyType {
    fn default() -> Self {
        Self::Nothing
    }
}
impl ValkyrieNode for RangeNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl Default for TupleKind {
    fn default() -> Self {
        Self::Tuple
    }
}

use super::*;
use crate::StatementBlock;

mod display;

/// `(a + b, c: d, ..e)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentsList {
    /// The raw string of the number.
    pub terms: Vec<TupleTermNode>,
}

/// `#annotation mut this: null`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentTerm {
    /// The modifier conditions
    pub modifiers: ModifiersNode,
    /// The key of the argument
    pub key: ArgumentKey,
    /// The value of the argument
    pub value: ExpressionNode,
}

/// The key of the argument
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ArgumentKey {
    /// `a + b`
    Nothing,
    /// `key: a + b`
    Symbol(IdentifierNode),
}

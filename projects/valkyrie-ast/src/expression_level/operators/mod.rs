use super::*;
use valkyrie_types::ValkyrieOperator;
mod display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrefixNode<E> {
    pub operator: OperatorNode,
    pub body: E,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InfixNode<E> {
    pub operator: OperatorNode,
    pub lhs: E,
    pub rhs: E,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PostfixNode<E> {
    pub operator: OperatorNode,
    pub body: E,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OperatorNode {
    pub kind: ValkyrieOperator,
    pub range: Range<usize>,
}

impl<E> PrefixNode<E> {
    pub fn update_range(mut self, range: Range<usize>) -> Self {
        self.range = range;
        self
    }
}

impl OperatorNode {
    pub fn new(kind: ValkyrieOperator, range: Range<usize>) -> Self {
        Self { kind, range }
    }
}

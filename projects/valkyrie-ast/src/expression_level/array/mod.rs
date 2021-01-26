#[cfg(feature = "pretty-print")]
mod display;

use super::*;
use crate::TupleKind;

/// The literal of array
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ArrayKind {
    /// `[1, 2:3, 4:5:6]`
    Ordinal,
    /// `⁅1, 2:3, 4:5:6⁆`
    Offset,
}

/// `[0, [], [:], [::]]`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayNode {
    ///  The kind of tuple.
    pub kind: ArrayKind,
    /// Terms
    pub terms: Vec<ArrayTermNode>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// `[index], ⁅start : end : step⁆`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ArrayTermNode {
    /// The index kind
    Index {
        /// The index of range
        index: ExpressionType,
    },
    /// The range
    Range {
        /// The first element in range
        head: Option<ExpressionType>,
        /// The middle element in range
        tail: Option<ExpressionType>,
        /// The
        step: Option<ExpressionType>,
    },
}

/// `array⁅index0⁆, array[index1]`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscriptCallNode {
    /// kind of
    pub kind: ArrayKind,
    /// `array`
    pub base: ExpressionType,
    /// `array?[0]`
    pub monadic: bool,
    /// `array[1, 2:3]`
    pub terms: Vec<ArrayTermNode>,
    /// The range of the node.
    pub span: Range<u32>,
}

impl Default for ArrayKind {
    fn default() -> Self {
        Self::Ordinal
    }
}

impl ArrayNode {
    /// Convert to tuple if possible
    pub fn as_tuple(&self) -> Option<TupleNode> {
        let mut terms = Vec::with_capacity(self.terms.len());
        for term in &self.terms {
            terms.push(term.as_tuple()?)
        }
        Some(TupleNode { kind: TupleKind::List, terms: vec![], span: self.span.clone() })
    }
}

impl ArrayTermNode {
    /// Convert to tuple item if possible
    pub fn as_tuple(&self) -> Option<TupleTermNode> {
        match self {
            ArrayTermNode::Index { index } => Some(TupleTermNode { pair: CallTermNode { key: None, value: index.clone() } }),
            ArrayTermNode::Range { .. } => None,
        }
    }
}

impl SubscriptCallNode {
    /// Replace placeholder with actual expression
    pub fn with_base(self, base: ExpressionType) -> Self {
        Self { base, ..self }
    }
    /// The linked method name
    pub fn method(&self) -> &'static str {
        match self.kind {
            ArrayKind::Ordinal => "subscript1",
            ArrayKind::Offset => "subscript0",
        }
    }
}

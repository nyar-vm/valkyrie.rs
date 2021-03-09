#[cfg(feature = "pretty-print")]
mod display;

use super::*;
use crate::{TupleKeyType, TupleKind};

/// The literal of array
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RangeKind {
    /// `[1, 2:3, 4:5:6]`
    Ordinal,
    /// `⁅1, 2:3, 4:5:6⁆`
    Offset,
}

/// `[0, [], [:], [::]]`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RangeNode {
    ///  The kind of tuple.
    pub kind: RangeKind,
    /// Terms
    pub terms: Vec<RangeTermNode>,
    /// The range of the number.
    pub span: Range<u32>,
}
impl ValkyrieNode for RangeNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
/// `[index], ⁅start : end : step⁆`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RangeTermNode {
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
    pub kind: RangeKind,
    /// `array`
    pub base: ExpressionType,
    /// `array?[0]`
    pub monadic: bool,
    /// `array[1, 2:3]`
    pub terms: Vec<RangeTermNode>,
    /// The range of the node.
    pub span: Range<u32>,
}
impl ValkyrieNode for SubscriptCallNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl Default for RangeKind {
    fn default() -> Self {
        Self::Ordinal
    }
}

impl RangeNode {
    /// Convert to tuple if possible
    pub fn as_tuple(&self) -> Option<TupleNode> {
        let mut terms = Vec::with_capacity(self.terms.len());
        for term in &self.terms {
            terms.push(term.as_tuple()?)
        }
        Some(TupleNode { kind: TupleKind::List, terms: vec![], span: self.span.clone() })
    }
}

impl RangeTermNode {
    /// Convert to tuple item if possible
    pub fn as_tuple(&self) -> Option<TupleTermNode> {
        match self {
            RangeTermNode::Index { index } => Some(TupleTermNode { key: TupleKeyType::Nothing, value: index.clone() }),
            RangeTermNode::Range { .. } => None,
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
            RangeKind::Ordinal => "subscript1",
            RangeKind::Offset => "subscript0",
        }
    }
}

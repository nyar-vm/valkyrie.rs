#[cfg(feature = "pretty-print")]
mod display;

use super::*;
use crate::{ArrayKind, ArrayTermNode};

/// `array⁅index0⁆` or `array[index1]`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscriptCallNode {
    /// kind of
    pub kind: ArrayKind,
    /// base of
    pub base: ExpressionType,
    /// The raw string of the number.
    pub terms: Vec<ArrayTermNode>,
    /// The range of the number.
    pub span: Range<u32>,
}

impl SubscriptCallNode {
    pub fn method(&self) -> &'static str {
        match self.kind {
            ArrayKind::Ordinal => "subscript1",
            ArrayKind::Offset => "subscript0",
        }
    }
}

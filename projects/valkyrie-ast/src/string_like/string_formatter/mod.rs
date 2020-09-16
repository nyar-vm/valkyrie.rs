use super::*;

/// `f"string %d %s %f"`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringFormatter {
    pub items: Vec<u32>,
    /// The range of the node
    pub span: Range<u32>,
}
use super::*;

/// `{lambda(args), ...}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaNode {
    pub arguments: Option<LambdaArgumentNode>,
}

/// `lambda(args)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaArgumentNode {
    /// The raw string of the number.
    pub terms: Vec<TopStatementNode>,
    /// The range of the number.
    pub range: Range<usize>,
}

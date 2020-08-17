use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchCaseNode {
    pub body: Vec<ExpressionBody>,
}

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ControlNode {
    pub r#type: ControlType,
    pub expression: Option<ExpressionNode>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ControlType {
    Break,
    Continue,
    Fallthrough,
    Return,
    Raise,
    Yield,
}

impl Display for ControlType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ControlType::Break => f.write_str("break"),
            ControlType::Continue => f.write_str("continue"),
            ControlType::Fallthrough => f.write_str("fallthrough"),
            ControlType::Return => f.write_str("return"),
            ControlType::Raise => f.write_str("raise"),
            ControlType::Yield => f.write_str("yield"),
        }
    }
}

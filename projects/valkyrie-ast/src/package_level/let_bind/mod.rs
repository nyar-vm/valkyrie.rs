use super::*;

mod display;

/// `let mut pattern = expression`
///
///
/// ```vk
/// let x;
/// let x: i32;
/// let x = 1;
/// let x: i32 = 1;
/// let mut x: i32 = 1;
/// let (x, ) = 1
/// let Some(x) = expr;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LetBindNode {
    pub pattern: PatternExpressionType,
    pub type_hint: Option<ExpressionNode>,
    pub body: Option<ExpressionNode>,
}

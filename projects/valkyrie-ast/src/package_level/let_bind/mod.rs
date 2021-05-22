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
pub struct VariableDeclaration {
    pub pattern: PatternNode,
    pub type_hint: Option<ExpressionKind>,
    pub body: Option<ExpressionNode>,
    pub span: Range<u32>,
}

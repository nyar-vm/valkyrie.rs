use crate::{ExpressionNode, IdentifierNode};
use alloc::vec::Vec;
use core::ops::Range;

/// `vow T {} fun f() { return T }`
///
///
/// ```vk
/// vouch T, U {
///     return T
///     effect [Asynchronous, Logging]
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GuaranteeNode {
    pub generics: Vec<IdentifierNode>,
    pub return_type: Option<ExpressionNode>,
    pub effect_type: Option<ExpressionNode>,
    pub span: Range<u32>,
}

/// `[Asynchronous<T>, Logging]`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EffectTypeNode {
    pub effect_type: Vec<ExpressionNode>,
    pub span: Range<u32>,
}

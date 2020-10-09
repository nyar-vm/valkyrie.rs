use super::*;
use crate::{control_flow::jmp_if::ThenStatement, StatementBody};
#[cfg(feature = "pretty-print")]
mod display;

/// `guard a > 0 else { ... }`
///
/// The else block must use control.
///
/// ```vk
/// 
/// assert a > 0 {
///     panic!("a must be greater than 0");
/// }
///
/// assert let Some(a) = b {
///     panic!("b must be Some");
/// }
///
///
///
/// if a < 0 {
///     return error;
/// }
/// do_something_else();
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GuardStatement {
    /// The condition to check
    pub condition: ExpressionNode,
    /// same as if condition
    pub main_body: GuardStatementBody,
    /// The range of the node
    pub span: Range<u32>,
}

/// `guard <CONDITION> then { ... } else { ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GuardStatementBody {
    /// Same as if condition
    Positive(ThenStatement),
    /// Same as if !condition
    Negative(ElseStatement),
}

/// `guard let Failure(error) = e then { ... }`
///
/// The else block must use control.
///
/// ```vk
/// if e.is_failure() {
///     let error = x.as_failure();
///     return error;
/// }
/// do_something_else();
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GuardLetStatement {
    /// The pattern to match
    pub pattern: PatternExpressionNode,
    /// The condition to check
    pub condition: ExpressionNode,
    /// same as if let condition
    pub then_body: ThenStatement,
    /// The range of the node
    pub span: Range<u32>,
}

impl GuardStatement {
    /// Get the last statement in the block
    pub fn last(&self) -> Option<&StatementBody> {
        let node = match &self.main_body {
            GuardStatementBody::Positive(node) => node.statements.last(),
            GuardStatementBody::Negative(node) => node.statements.last(),
        }?;
        Some(&node.r#type)
    }
}

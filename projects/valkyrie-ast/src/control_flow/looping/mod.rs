use super::*;
use crate::FunctionStatementNode;
use alloc::vec::Vec;
use core::fmt::{Debug, Display, Formatter};

/// `loop { ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LoopStatementNode {
    pub body: Vec<FunctionStatementNode>,
    pub eos: bool,
    pub range: Range<usize>,
}

impl Display for LoopStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("loop { ")?;
        for statement in &self.body {
            Display::fmt(statement, f)?;
        }
        f.write_str(" }")
    }
}

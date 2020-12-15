use crate::{
    control_flow::jmp_if::ThenStatement, ElseStatement, ExpressionNode, PatternBranch, PatternExpressionType, PatternGuard,
    StatementBlock, StatementNode, StatementType, SwitchStatement, WhileConditionNode,
};
use alloc::{boxed::Box, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};
#[cfg(feature = "pretty-print")]
use pretty_print::{
    helpers::{PrettySequence, SoftBlock},
    PrettyBuilder, PrettyPrint, PrettyProvider, PrettyTree,
};

pub mod control;
pub mod jmp_guard;
pub mod jmp_if;
pub mod jmp_switch;
pub mod loop_for;
pub mod loop_pure;
pub mod loop_while;

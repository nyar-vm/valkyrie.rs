use crate::{
    ElseStatement, ExpressionNode, ImplicitCaseNode, PatternBranch, PatternExpressionNode, PatternGuard, StatementBlock,
    StatementNode, SwitchStatement, WhileConditionNode,
};
use alloc::{boxed::Box, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};
use deriver::From;
use pretty_print::PrettyBuilder;
#[cfg(feature = "pretty-print")]
use pretty_print::{helpers::PrettySequence, PrettyPrint, PrettyProvider, PrettyTree};

pub mod control;
pub mod guard_statement;
pub mod jmp_if;
pub mod jmp_switch;
pub mod loop_for;
pub mod loop_while;

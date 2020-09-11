use crate::{
    ArgumentKeyNode, ElseStatement, ExpressionNode, IfConditionNode, PatternExpression, PatternGuard, StatementBlock,
    StatementNode, SwitchStatement, WhileConditionNode,
};
use alloc::{boxed::Box, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use deriver::From;
#[cfg(feature = "pretty-print")]
use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};

pub mod control;
pub mod guard_statement;
pub mod jmp_if;
pub mod jmp_switch;
pub mod loop_for;
pub mod loop_while;
pub mod pattern;

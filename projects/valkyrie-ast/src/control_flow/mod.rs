use crate::{
    ElseStatement, ExpressionNode, LetBindNode, PatternBranch, PatternExpression, PatternGuard, StatementBlock, StatementNode,
    SwitchStatement, TuplePatternNode, WhileConditionNode,
};
use alloc::{boxed::Box, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};
#[cfg(feature = "pretty-print")]
use pretty_print::{helpers::PrettySequence, PrettyPrint, PrettyProvider, PrettyTree};
use pretty_print::{helpers::SoftBlock, PrettyBuilder};

pub mod control;
pub mod jmp_guard;
pub mod jmp_if;
pub mod jmp_switch;
pub mod loop_for;
pub mod loop_pure;
pub mod loop_while;

#[cfg(feature = "pretty-print")]
use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};

use crate::{ConditionType, ExpressionNode, FunctionBody, StatementNode};
use alloc::{boxed::Box, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

pub mod control;
pub mod guard_statement;
pub mod if_else;
pub mod looping;

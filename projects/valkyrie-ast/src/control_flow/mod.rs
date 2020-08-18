#[cfg(feature = "pretty-print")]
use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};

use crate::{ConditionType, ExpressionNode, FunctionBody, StatementNode};
use alloc::{borrow::Cow, boxed::Box, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

pub mod control;
pub mod for_loop;
pub mod if_else;
pub mod looping;

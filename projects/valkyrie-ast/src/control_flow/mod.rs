use crate::{control_flow::if_else::format_else_body, ConditionType, ExpressionNode, StatementNode};
use core::{
    fmt::{Debug, Display, Formatter, Write},
    ops::Range,
};
use indentation::{IndentDisplay, IndentFormatter};
use std::{boxed::Box, vec::Vec};

pub mod control;
pub mod for_loop;
pub mod if_else;
pub mod looping;

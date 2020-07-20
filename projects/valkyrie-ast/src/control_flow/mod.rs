use crate::{
    control_flow::if_else::format_else_body, ConditionType, ExpressionBody, ExpressionNode, ExpressionType, StatementNode,
};
use alloc::{boxed::Box, vec::Vec};
use core::{
    borrow::BorrowMut,
    fmt::{Debug, Display, Formatter, Write},
    ops::Range,
};
use indentation::{IndentDisplay, IndentFormatter};

pub mod control;
pub mod for_loop;
pub mod if_else;
pub mod looping;

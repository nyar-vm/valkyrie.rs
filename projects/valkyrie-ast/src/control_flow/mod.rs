use crate::{ExpressionNode, LambdaArgumentNode, StatementNode};
use alloc::{boxed::Box, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

pub mod for_loop;
pub mod if_else;
pub mod looping;

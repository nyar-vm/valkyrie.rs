use crate::{ConditionType, ExpressionNode, PrettyPrint, PrettyProvider, PrettyTree, StatementNode};
use core::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};
use pretty::{DocAllocator, RefDoc};
use std::{boxed::Box, vec::Vec};

pub mod control;
pub mod for_loop;
pub mod if_else;
pub mod looping;

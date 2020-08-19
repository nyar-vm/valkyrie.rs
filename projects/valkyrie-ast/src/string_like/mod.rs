use crate::{ExpressionBody, IdentifierNode};
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};
use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};

pub mod string_formatter;
pub mod string_literal;
pub mod string_template;

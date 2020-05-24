mod display;
mod parser;

use regex::Regex;
use std::sync::LazyLock;

use crate::expression::ValkyrieExpression;
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};
use std::{
    fmt::{Display, Formatter},
    ops::Range,
    str::FromStr,
};

/// These names cannot be used as function names
pub const KEYWORDS: [&str; 3] = ["true", "false", "null"];

impl From<ValkyrieNamepath> for ValkyrieExpression {
    fn from(value: ValkyrieNamepath) -> Self {
        ValkyrieExpression::Symbol(Box::new(value))
    }
}

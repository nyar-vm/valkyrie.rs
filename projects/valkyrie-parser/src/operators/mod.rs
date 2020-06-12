use crate::traits::ThisParser;
use lispify::Lisp;
use pratt::{Associativity, Precedence};

use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    str::FromStr,
    sync::LazyLock,
};
use valkyrie_ast::{OperatorNode, ValkyrieOperator};
use valkyrie_types::third_party::pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, Regex, StopBecause,
};

mod display;
mod parser;

#[derive(Clone)]
pub struct ValkyrieInfix {
    normalized: String,
    range: Range<usize>,
}

#[derive(Clone)]
pub struct ValkyriePrefix {
    normalized: String,
    range: Range<usize>,
}

#[derive(Clone)]
pub struct ValkyrieSuffix {
    normalized: String,
    range: Range<usize>,
}

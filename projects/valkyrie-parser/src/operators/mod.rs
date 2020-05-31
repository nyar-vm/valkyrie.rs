use crate::traits::ThisParser;
use lispify::Lisp;
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};
use pratt::{Associativity, Precedence};
use regex::Regex;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    str::FromStr,
    sync::LazyLock,
};
use valkyrie_ast::{OperatorKind, OperatorNode};

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

use crate::traits::ValkyrieParser;
use lispify::{Lisp, Lispify};
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};
use pratt::{Associativity, Precedence};
use regex::Regex;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
    str::FromStr,
    sync::LazyLock,
};
use valkyrie_ast::{ValkyrieOperator, ValkyrieOperatorKind};

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

impl ValkyrieParser for ValkyrieOperator {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn parse_many(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

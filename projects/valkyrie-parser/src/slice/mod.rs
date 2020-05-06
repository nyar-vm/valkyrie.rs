mod display;
mod parser;
use crate::{expression::ValkyrieExpression, helpers::ignore, number::ValkyrieNumber, symbol::ValkyrieIdentifier};
use lispify::{Lisp, LispNumber, Lispify};
use pex::{
    helpers::{make_from_str, whitespace},
    BracketPattern, ParseResult, ParseState, StopBecause,
};
use regex::Regex;
use std::{
    fmt::{Display, Formatter},
    ops::Range,
    str::FromStr,
    sync::LazyLock,
};

/// A number literal.
#[derive(Debug, Clone)]
pub struct ValkyrieSlice {
    /// The raw string of the number.
    pub base: ValkyrieExpression,
    /// The raw string of the number.
    pub terms: Vec<ValkyrieSliceTerm>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// A number literal.
#[derive(Debug, Clone)]
pub struct ValkyrieSliceTerm {
    /// The raw string of the number.
    pub is_index: bool,
    /// The raw string of the number.
    pub start: Option<ValkyrieExpression>,
    /// The unit of the number, if any.
    pub end: Option<ValkyrieExpression>,
    /// The unit of the number, if any.
    pub step: Option<ValkyrieExpression>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl PartialEq for ValkyrieSliceTerm {
    fn eq(&self, other: &Self) -> bool {
        self.start.eq(&other.start) && self.end.eq(&other.end) && self.step.eq(&other.step)
    }
}

impl PartialEq for ValkyrieSlice {
    fn eq(&self, other: &Self) -> bool {
        self.terms.eq(&other.terms)
    }
}

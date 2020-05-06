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
pub enum ValkyrieSliceTerm {
    Single {
        element: ValkyrieExpression,
        /// The range of the number.
        range: Range<usize>,
    },
    Ranged {
        /// The raw string of the number.
        start: Option<ValkyrieExpression>,
        /// The unit of the number, if any.
        end: Option<ValkyrieExpression>,
        /// The unit of the number, if any.
        step: Option<ValkyrieExpression>,
        /// The range of the number.
        range: Range<usize>,
    },
}

impl PartialEq for ValkyrieSliceTerm {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Single { element, .. }, Self::Single { element: other, .. }) => element.eq(other),
            (
                Self::Ranged { start, end, step, .. },
                Self::Ranged { start: other_start, end: other_end, step: other_step, .. },
            ) => start.eq(other_start) && end.eq(other_end) && step.eq(other_step),
            _ => false,
        }
    }
}

impl PartialEq for ValkyrieSlice {
    fn eq(&self, other: &Self) -> bool {
        self.terms.eq(&other.terms)
    }
}

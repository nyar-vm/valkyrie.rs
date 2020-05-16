mod display;
mod parser;
use crate::{expression::ValkyrieExpression, helpers::ignore};
use lispify::{Lisp, Lispify};
use pex::{
    helpers::{make_from_str, whitespace},
    BracketPattern, ParseResult, ParseState, StopBecause,
};

use std::{
    fmt::{Display, Formatter},
    ops::Range,
    str::FromStr,
};

/// A number literal.
#[derive(Debug, Clone)]
pub struct ValkyrieView {
    /// The raw string of the number.
    pub base: ValkyrieExpression,
    /// The raw string of the number.
    pub terms: Vec<ValkyrieViewTerm>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// A number literal.
#[derive(Debug, Clone)]
pub enum ValkyrieViewTerm {
    /// `array[index]`, also can be a call_index `array[[1, 2, 3]]`
    Index {
        element: ValkyrieExpression,
        /// The range of the number.
        range: Range<usize>,
    },
    /// `a[start:end:step]`
    Range {
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

impl PartialEq for ValkyrieViewTerm {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Index { element, .. }, Self::Index { element: other, .. }) => element.eq(other),
            (
                Self::Range { start, end, step, .. },
                Self::Range { start: other_start, end: other_end, step: other_step, .. },
            ) => start.eq(other_start) && end.eq(other_end) && step.eq(other_step),
            _ => false,
        }
    }
}

impl PartialEq for ValkyrieView {
    fn eq(&self, other: &Self) -> bool {
        self.terms.eq(&other.terms)
    }
}

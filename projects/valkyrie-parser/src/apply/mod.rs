mod display;
mod parser;
use crate::{expression::ValkyrieExpression, helpers::ignore, symbol::ValkyrieIdentifier};
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
pub struct ValkyrieApply {
    /// The raw string of the number.
    pub base: ValkyrieExpression,
    /// The raw string of the number.
    pub terms: Vec<ValkyrieTableTerm>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// A number literal.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValkyrieTableTerm {
    /// `array[index]`, also can be a slice `array[[1, 2, 3]]`
    Item(ValkyrieExpression),
    /// `a[start:end:step]`
    Pair(ValkyriePair),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValkyriePair {
    pub key: ValkyrieIdentifier,
    pub value: ValkyrieExpression,
}

impl ValkyriePair {
    pub fn get_range(&self) -> Range<usize> {
        self.key.range.start..self.value.get_range().end
    }
}

impl PartialEq for ValkyrieApply {
    fn eq(&self, other: &Self) -> bool {
        self.terms.eq(&other.terms)
    }
}

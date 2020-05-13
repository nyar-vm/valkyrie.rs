mod display;
mod parser;
use crate::{expression::ValkyrieExpression, helpers::ignore, symbol::ValkyrieIdentifier};
use lispify::{Lisp, Lispify};
use pex::{
    helpers::{make_from_str, whitespace},
    BracketPattern, ParseResult, ParseState, StopBecause,
};

use crate::apply::{ValkyrieApply, ValkyrieTableTerm};
use std::{
    fmt::{Display, Formatter},
    ops::Range,
    str::FromStr,
};

/// A number literal.
#[derive(Debug, Clone)]
pub struct ValkyrieDotCall {
    /// The raw string of the number.
    pub base: ValkyrieExpression,
    /// The raw string of the number.
    pub caller: ValkyrieIdentifier,
    /// The range of the number.
    pub terms: Vec<ValkyrieTableTerm>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl PartialEq for ValkyrieDotCall {
    fn eq(&self, other: &Self) -> bool {
        self.terms.eq(&other.terms) && self.caller.eq(&other.caller) && self.base.eq(&other.base)
    }
}

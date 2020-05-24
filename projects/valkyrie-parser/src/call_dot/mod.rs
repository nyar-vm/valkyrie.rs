mod display;
mod parser;
use crate::{expression::ValkyrieExpression, helpers::ignore};
use lispify::{Lisp, Lispify};
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};

use crate::{call_apply::ValkyrieApply, table::ValkyrieTableTerm};
use std::{
    fmt::{Display, Formatter},
    ops::Range,
    str::FromStr,
};
use valkyrie_ast::ValkyrieIdentifier;

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

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
use valkyrie_ast::TableTermNode;

/// A number literal.
#[derive(Debug, Clone)]
pub struct ValkyrieApply {
    /// The raw string of the number.
    pub base: ValkyrieExpression,
    /// The raw string of the number.
    pub terms: Vec<TableTermNode<ValkyrieExpression>>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl PartialEq for ValkyrieApply {
    fn eq(&self, other: &Self) -> bool {
        self.terms.eq(&other.terms)
    }
}

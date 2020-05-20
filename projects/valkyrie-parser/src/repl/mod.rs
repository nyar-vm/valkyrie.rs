mod display;
mod parser;
use crate::{expression::ValkyrieExpression, helpers::ignore};
use lispify::{Lisp, Lispify};
use pex::{ParseResult, ParseState};

pub use self::parser::parse_repl;
use std::fmt::{Display, Formatter};
/// A number literal.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValkyrieREPL {
    Expression(Box<ValkyrieExpression>),
}

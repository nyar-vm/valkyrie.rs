mod display;
mod parser;
use crate::helpers::ignore;
use lispify::{Lisp, Lispify};
use pex::{helpers::whitespace, BracketPattern, ParseResult, ParseState, StopBecause};

use crate::expression::ValkyrieExpression;
use std::{fmt::Formatter, ops::Range, str::FromStr};
use valkyrie_ast::{TableNode, TableTermNode};

impl From<TableNode<ValkyrieExpression>> for ValkyrieExpression {
    fn from(value: TableNode<ValkyrieExpression>) -> Self {
        ValkyrieExpression::Table(Box::new(value))
    }
}

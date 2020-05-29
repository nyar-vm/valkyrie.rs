mod display;
mod parser;
use crate::helpers::ignore;
use lispify::{Lisp, Lispify};
use pex::{
    helpers::{make_from_str, whitespace},
    BracketPattern, ParseResult, ParseState, StopBecause,
};

use crate::expression::ValkyrieExpression;
use std::{
    fmt::{Display, Formatter},
    ops::Range,
    str::FromStr,
};
use valkyrie_ast::IdentifierNode;

/// A number literal.
#[derive(Debug, Clone, Eq)]
pub struct TableNode {
    pub index0: bool,
    /// The raw string of the number.
    pub terms: Vec<TableTermNode>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl From<TableNode> for ValkyrieExpression {
    fn from(value: TableNode) -> Self {
        ValkyrieExpression::Table(Box::new(value))
    }
}

impl Display for TableNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.index0 {
            write!(f, "[")?;
        }
        else {
            write!(f, "[[")?;
        }
        for (i, term) in self.terms.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            match term {
                TableTermNode::Item(item) => write!(f, "{}", item)?,
                TableTermNode::Pair(pair) => write!(f, "{}: {}", pair.key, pair.value)?,
            }
        }
        if self.index0 { write!(f, "]") } else { write!(f, "]]") }
    }
}

impl PartialEq for TableNode {
    fn eq(&self, other: &Self) -> bool {
        self.terms.eq(&other.terms)
    }
}

/// A number literal.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TableTermNode {
    /// `array[index]`, also can be a call_index `array[[1, 2, 3]]`
    Item(ValkyrieExpression),
    /// `a[start:end:step]`
    Pair(PairNode),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PairNode {
    pub key: IdentifierNode,
    pub value: ValkyrieExpression,
}

impl PairNode {
    pub fn get_range(&self) -> Range<usize> {
        self.key.span.start..self.value.get_range().end
    }
}

use crate::{helpers::ignore, traits::ThisParser};
use lispify::{Lisp, Lispify};
use std::ops::Range;
use valkyrie_ast::{CallTermNode, ExpressionBody, IdentifierNode, TableKind, TableNode, TableTermNode};
use valkyrie_types::third_party::pex::{BracketPattern, ParseResult, ParseState};

mod table;
mod tuple;

pub(crate) struct TupleNode {
    /// The raw string of the number.
    pub terms: Vec<TableTermNode>,
    /// The range of the number.
    pub range: Range<usize>,
}

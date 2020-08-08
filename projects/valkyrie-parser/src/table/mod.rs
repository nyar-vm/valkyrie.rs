mod table;
mod tuple;

use crate::{helpers::ignore, traits::ThisParser, utils::get_span};
use lispify::{Lisp, Lispify};
use std::ops::Range;
use valkyrie_ast::{
    CallTermNode, ExpressionBody, IdentifierNode, NumberLiteralNode, StringLiteralNode, SubscriptNode, TableKeyType, TableKind,
    TableNode, TableTermNode,
};
use valkyrie_types::third_party::pex::{BracketPattern, ParseResult, ParseState};

pub(crate) struct TupleNode {
    /// The raw string of the number.
    pub terms: Vec<TableTermNode>,
    /// The range of the number.
    pub span: Range<u32>,
}

impl From<TupleNode> for ExpressionBody {
    fn from(value: TupleNode) -> Self {
        ExpressionBody::Table(Box::new(value.as_table()))
    }
}

use crate::{helpers::ignore, traits::ThisParser};
use lispify::{Lisp, Lispify};
use std::ops::Range;
use valkyrie_ast::{MaybePair, ExpressionBody, IdentifierNode, TableKind, TableNode};
use valkyrie_types::third_party::pex::{BracketPattern, ParseResult, ParseState};

mod table;
mod tuple;


pub(crate) struct TupleNode<E> {
    /// The raw string of the number.
    pub terms: Vec<MaybePair<IdentifierNode, E>>,
    /// The range of the number.
    pub range: Range<usize>,
}

mod table;
mod tuple;

use crate::{helpers::ignore, traits::ThisParser, utils::get_span};
use lispify::Lisp;
use pex::{BracketPattern, ParseResult, ParseState};
use std::ops::Range;
use valkyrie_ast::{
    CallTermNode, ExpressionType, IdentifierNode, NumberLiteralNode, StringLiteralNode, SubscriptNode, TupleKeyType, TupleKind,
    TupleNode, TupleTermNode,
};

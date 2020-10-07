mod raw;
mod template;

use crate::{utils::get_span, ThisParser};
use lispify::Lisp;
use pex::{
    helpers::{quotation_pair, quotation_pair_nested},
    ParseResult, ParseState, Regex,
};
use std::sync::LazyLock;
use valkyrie_ast::{IdentifierNode, StatementNode, StringLiteralNode, StringTemplateNode};

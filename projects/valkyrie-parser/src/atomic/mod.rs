use crate::{
    helpers::{ignore, parse_name_join},
    traits::ThisParser,
    utils::get_span,
};
use lispify::{Lisp, LispSymbol};
use std::sync::LazyLock;
use valkyrie_ast::{IdentifierNode, MacroPathNode, NamePathNode, PrettyPrint};
use valkyrie_types::third_party::pex::{ParseResult, ParseState, Regex};

mod identifier;
mod macro_name;
mod namepath;

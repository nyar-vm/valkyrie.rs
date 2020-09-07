use crate::{helpers::parse_name_join, traits::ThisParser, utils::get_span};
use lispify::{Lisp, LispNumber, ListString};
use pex::{
    helpers::{quotation_pair, quotation_pair_nested},
    ParseResult, ParseState, Regex, StopBecause,
};
use std::{ops::Range, sync::LazyLock};
use valkyrie_ast::{IdentifierNode, NamePathNode, NumberLiteralNode, PrettyPrint, StringLiteralNode};

mod bytes;
mod identifier;
mod namepath;
mod number;
mod string;
mod string_template;

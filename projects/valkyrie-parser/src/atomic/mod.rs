use crate::{helpers::parse_name_join, traits::ThisParser, utils::get_span};
use lispify::Lisp;
use pex::{ParseResult, ParseState, Regex, StopBecause};
use std::{ops::Range, sync::LazyLock};
use valkyrie_ast::{IdentifierNode, NamePathNode, NumberLiteralNode, PrettyPrint};

mod bytes;
mod identifier;
mod namepath;
mod number;

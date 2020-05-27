mod display;
mod parser;

use regex::Regex;
use std::sync::LazyLock;

use crate::expression::ValkyrieExpression;
use pex::{ParseResult, ParseState};

use valkyrie_ast::NamepathNode;

/// These names cannot be used as function names
pub const KEYWORDS: [&str; 3] = ["true", "false", "null"];

impl From<NamepathNode> for ValkyrieExpression {
    fn from(value: NamepathNode) -> Self {
        ValkyrieExpression::Symbol(Box::new(value))
    }
}

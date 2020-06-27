mod display;
mod parser;
use crate::helpers::ignore;
use lispify::{Lisp, Lispify};
use valkyrie_types::third_party::pex::{ParseResult, ParseState};

pub use self::parser::parse_repl;
use std::fmt::{Display, Formatter};
use valkyrie_ast::{NamespaceDeclarationNode, TermExpressionNode};

/// A number literal.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValkyrieREPL {
    Namespace(Box<NamespaceDeclarationNode>),
    Expression(Box<TermExpressionNode>),
}

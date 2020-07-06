use crate::{helpers::ignore, traits::ThisParser};
use lispify::Lisp;

use crate::helpers::parse_eos;
use valkyrie_ast::{
    ClassDeclarationNode, ExpressionNode, ImportStatementNode, NamespaceDeclarationNode, StatementNode, StatementType,
};
use valkyrie_types::third_party::pex::{ParseResult, ParseState};

mod lambda;
mod normal;
pub mod repl;
pub mod script;
use crate::{helpers::ignore, looping::FunctionBody, ThisParser};
use lispify::Lisp;
use valkyrie_ast::{
    ApplyArgumentNode, ArgumentKeyNode, ArgumentTermNode, FunctionDeclarationNode, FunctionType, IdentifierNode, NamePathNode,
};
use valkyrie_types::third_party::pex::{BracketPair, BracketPattern, ParseResult, ParseState, StopBecause};

mod function;

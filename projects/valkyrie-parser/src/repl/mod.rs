use crate::{helpers::ignore, traits::ThisParser};
use lispify::{Lisp, Lispify};
use std::fmt::{Display, Formatter};
use valkyrie_ast::{
    ClassDeclarationNode, LoopStatementNode, NamespaceDeclarationNode, ReplStatementNode, TermExpressionNode,
    TermExpressionType, TopStatementNode,
};
use valkyrie_types::third_party::pex::{ParseResult, ParseState};

#[track_caller]
pub fn parse_repl(s: &str) -> Vec<ReplStatementNode> {
    let input = ParseState::new(s);
    let (state, repl) = match input.match_repeats(ReplStatementNode::parse) {
        ParseResult::Pending(s, v) => (s.skip(ignore), v),
        ParseResult::Stop(e) => panic!("Failed to parse REPL: {:?}", e),
    };
    if !state.residual.is_empty() {
        panic!("Expect EOF, found:\n{}", state.residual)
    }
    repl
}

impl ThisParser for ReplStatementNode {
    /// - [term](TermExpressionType::parse)
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, expr) = input
            .skip(ignore)
            .begin_choice()
            .or_else(|s| LoopStatementNode::parse(s).map_inner(Into::into))
            .or_else(|s| TermExpressionNode::parse(s).map_inner(Into::into))
            .end_choice()?;
        let (state, _) = state.skip(ignore).match_optional(|s| s.match_char(';'))?;
        state.finish(expr)
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            ReplStatementNode::DeclareClass(v) => v.as_lisp(),
            ReplStatementNode::Expression(v) => v.as_lisp(),
            ReplStatementNode::Loop(v) => v.as_lisp(),
        }
    }
}

impl ThisParser for TopStatementNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, expr) = input
            .skip(ignore)
            .begin_choice()
            .or_else(|s| NamespaceDeclarationNode::parse(s).map_inner(Into::into))
            .or_else(|s| TermExpressionNode::parse(s).map_inner(Into::into))
            .end_choice()?;
        let (state, _) = state.skip(ignore).match_optional(|s| s.match_char(';'))?;
        state.finish(expr)
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            TopStatementNode::DeclareNamespace(v) => v.as_lisp(),
            TopStatementNode::DeclareClass(v) => v.as_lisp(),
            TopStatementNode::Expression(v) => v.as_lisp(),
        }
    }
}

impl ThisParser for ClassDeclarationNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("class")?;
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

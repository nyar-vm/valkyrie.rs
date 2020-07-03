use crate::{helpers::ignore, traits::ThisParser};
use lispify::Lisp;

use valkyrie_ast::{
    ClassDeclarationNode, ExpressionNode, ImportStatementNode, LoopStatementNode, NamespaceDeclarationNode, ReplStatementNode,
    TopStatementNode,
};
use valkyrie_types::third_party::pex::{ParseResult, ParseState};

impl ThisParser for ReplStatementNode {
    /// - [term](ExpressionType::parse)
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, expr) = input
            .skip(ignore)
            .begin_choice()
            .or_else(|s| LoopStatementNode::parse(s).map_inner(Into::into))
            .or_else(|s| ExpressionNode::parse(s).map_inner(Into::into))
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
            .or_else(|s| ImportStatementNode::parse(s).map_inner(Into::into))
            .or_else(|s| ExpressionNode::parse(s).map_inner(Into::into))
            .end_choice()?;
        let (state, _) = state.skip(ignore).match_optional(|s| s.match_char(';'))?;
        state.finish(expr)
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            TopStatementNode::Namespace(v) => v.as_lisp(),
            TopStatementNode::Class(v) => v.as_lisp(),
            TopStatementNode::Expression(v) => v.as_lisp(),
            TopStatementNode::Import(v) => v.as_lisp(),
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

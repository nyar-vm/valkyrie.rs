use crate::{
    helpers::{ignore, parse_any_name_path, parse_eos, parse_name_join_dot},
    traits::ThisParser,
    utils::{get_span, parse_expression_node},
};
use lispify::{Lisp, ListString};
use pex::{helpers::comment_line, BracketPattern, ParseResult, ParseState, Regex, StopBecause};
use std::sync::LazyLock;
use valkyrie_ast::{
    ApplyCallNode, ClassDeclaration, ControlNode, DocumentationNode, ExpressionContext, ExpressionNode, FlagsDeclaration,
    ForLoop, FunctionDeclaration, GenericCallNode, GuardPattern, GuardStatement, IdentifierNode, ImportAliasNode,
    ImportGroupNode, ImportStatementNode, ImportTermNode, LambdaArgumentNode, LambdaNode, LetBindNode, NamePathNode,
    NamespaceDeclaration, NamespaceKind, NewConstructNode, PatternType, StatementBlock, StatementBody, StatementNode,
    TableTermNode, TypingExpression, UnionDeclaration, WhileLoop,
};

mod classes;
mod def_var;
mod documentation;
mod guard;
mod import;
mod lambda;
mod new;

pub struct ReplRoot {
    pub statements: Vec<StatementNode>,
}

pub struct ScriptRoot {
    pub statements: Vec<StatementNode>,
}

impl ThisParser for ReplRoot {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.match_repeats(|s| parse_statement_node(s, false)).map_inner(|s| ReplRoot { statements: s })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ThisParser for ScriptRoot {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.match_repeats(|s| parse_statement_node(s, false)).map_inner(|s| ScriptRoot { statements: s })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ThisParser for StatementNode {
    /// - [term](ExpressionType::parse)
    fn parse(input: ParseState) -> ParseResult<Self> {
        parse_statement_node(input, false)
    }

    fn as_lisp(&self) -> Lisp {
        self.r#type.as_lisp()
    }
}

pub fn parse_statement_node(input: ParseState, repl: bool) -> ParseResult<StatementNode> {
    let parser = match repl {
        true => parse_repl_statements,
        false => StatementBody::parse,
    };
    let (state, expr) = input.skip(ignore).match_fn(parser)?;
    let (state, eos) = parse_eos(state)?;
    state.finish(StatementNode { r#type: expr, end_semicolon: eos, span: get_span(input, state) })
}

impl ThisParser for StatementBody {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| NamespaceDeclaration::parse(s).map_inner(Into::into))
            .or_else(|s| ImportStatementNode::parse(s).map_inner(Into::into))
            .or_else(|s| ClassDeclaration::parse(s).map_inner(Into::into))
            .or_else(|s| UnionDeclaration::parse(s).map_inner(Into::into))
            .or_else(|s| FlagsDeclaration::parse(s).map_inner(Into::into))
            .or_else(function_with_head)
            .or_else(|s| LetBindNode::parse(s).map_inner(Into::into))
            .or_else(|s| GuardStatement::parse(s).map_inner(Into::into))
            .or_else(|s| WhileLoop::parse(s).map_inner(Into::into))
            .or_else(|s| ForLoop::parse(s).map_inner(Into::into))
            .or_else(|s| ControlNode::parse(s).map_inner(Into::into))
            .or_else(|s| parse_expression_node(s, ExpressionContext::in_free()).map_inner(Into::into))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            StatementBody::Nothing => Lisp::Any(vec![]),
            StatementBody::Namespace(v) => v.as_lisp(),
            StatementBody::Import(v) => v.as_lisp(),
            StatementBody::While(v) => v.as_lisp(),
            StatementBody::For(v) => v.as_lisp(),
            StatementBody::Class(v) => v.as_lisp(),
            StatementBody::ClassField(v) => v.as_lisp(),
            StatementBody::Expression(v) => v.as_lisp(),
            StatementBody::Function(v) => v.as_lisp(),
            StatementBody::Control(v) => v.as_lisp(),
            StatementBody::Document(v) => v.as_lisp(),
            StatementBody::LetBind(v) => v.as_lisp(),
            StatementBody::Guard(v) => v.as_lisp(),
            StatementBody::Flags(v) => v.as_lisp(),
            StatementBody::FlagsField(v) => v.as_lisp(),
            StatementBody::Union(v) => v.as_lisp(),
            StatementBody::Variant(v) => v.as_lisp(),
        }
    }
}

pub fn parse_repl_statements(input: ParseState) -> ParseResult<StatementBody> {
    input
        .begin_choice()
        .or_else(|s| NamespaceDeclaration::parse(s).map_inner(Into::into))
        .or_else(|s| ImportStatementNode::parse(s).map_inner(Into::into))
        .or_else(|s| ClassDeclaration::parse(s).map_inner(Into::into))
        .or_else(|s| LetBindNode::parse(s).map_inner(Into::into))
        .or_else(|s| FunctionDeclaration::parse(s).map_inner(Into::into))
        .or_else(|s| WhileLoop::parse(s).map_inner(Into::into))
        .or_else(|s| ForLoop::parse(s).map_inner(Into::into))
        .or_else(|s| parse_expression_node(s, ExpressionContext::in_free()).map_inner(Into::into))
        .end_choice()
}

fn function_with_head(input: ParseState) -> ParseResult<StatementBody> {
    let (state, mods) = input.match_repeats(|s| {
        let (state, _) = s.skip(ignore).match_negative(FunctionDeclaration::parse, "KW_FUNCTION")?;
        IdentifierNode::parse(state)
    })?;
    let (state, mut func) = state.skip(ignore).match_fn(FunctionDeclaration::parse)?;
    func.modifiers = mods;
    state.finish(StatementBody::Function(Box::new(func)))
}

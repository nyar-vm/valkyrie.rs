use super::*;
use valkyrie_ast::{ForLoopNode, FunctionDeclarationNode, NamePathNode, WhileLoopNode};

impl ThisParser for StatementNode {
    /// - [term](ExpressionType::parse)
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, expr) = input.skip(ignore).match_fn(StatementType::parse)?;
        let (state, eos) = parse_eos(state)?;
        state.finish(StatementNode { r#type: expr, eos, range: state.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        self.r#type.as_lisp()
    }
}

impl ThisParser for StatementType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, expr) = input
            .begin_choice()
            .or_else(|s| NamespaceDeclarationNode::parse(s).map_inner(Into::into))
            .or_else(|s| ImportStatementNode::parse(s).map_inner(Into::into))
            .or_else(|s| ClassDeclarationNode::parse(s).map_inner(Into::into))
            .or_else(|s| FunctionDeclarationNode::parse(s).map_inner(Into::into))
            .or_else(|s| WhileLoopNode::parse(s).map_inner(Into::into))
            .or_else(|s| ForLoopNode::parse(s).map_inner(Into::into))
            .or_else(|s| ExpressionNode::parse(s).map_inner(Into::into))
            .end_choice()?;
        let (state, _) = state.skip(ignore).match_optional(|s| s.match_char(';'))?;
        state.finish(expr)
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            StatementType::Nothing => Lisp::Any(vec![]),
            StatementType::Namespace(v) => v.as_lisp(),
            StatementType::Import(v) => v.as_lisp(),
            StatementType::While(v) => v.as_lisp(),
            StatementType::For(v) => v.as_lisp(),
            StatementType::Class(v) => v.as_lisp(),
            StatementType::Expression(v) => v.as_lisp(),
            StatementType::Function(v) => v.as_lisp(),
        }
    }
}

impl ThisParser for ClassDeclarationNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("class")?;
        let (state, namepath) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (state, _) = state.skip(ignore).match_char('{')?;
        let (state, _) = state.skip(ignore).match_char('}')?;
        state.finish(ClassDeclarationNode {
            namepath,
            modifiers: vec![],
            extends: None,
            implements: vec![],
            statements: vec![],
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut items = Vec::with_capacity(4);
        items.push(Lisp::keyword("class"));
        items.push(self.namepath.as_lisp());
        Lisp::Any(items)
    }
}

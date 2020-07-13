use crate::{helpers::ignore, ThisParser};
use lispify::Lisp;
use valkyrie_ast::{ApplyArgumentNode, ArgumentTermNode, FunctionDeclarationNode, FunctionType, NamePathNode};
use valkyrie_types::third_party::pex::{ParseResult, ParseState, StopBecause};

impl ThisParser for FunctionDeclarationNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, head) = FunctionType::parse(input)?;
        let (state, name) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (state, args) = state.skip(ignore).match_fn(ApplyArgumentNode::parse)?;

        state.finish(FunctionDeclarationNode {
            r#type: head,
            namepath: name,
            modifiers: vec![],
            extends: None,
            implements: vec![],
            statements: vec![],
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut items = vec![Lisp::keyword(self.r#type.to_string()), self.namepath.as_lisp()];

        Lisp::Any(items)
    }
}

impl ThisParser for FunctionType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        if input.residual.starts_with("def") {
            input.advance("def").finish(FunctionType::Micro)
        }
        else if input.residual.starts_with("micro") {
            input.advance("micro").finish(FunctionType::Macro)
        }
        else if input.residual.starts_with("macro") {
            input.advance("macro").finish(FunctionType::Macro)
        }
        else {
            StopBecause::must_be("def", input.start_offset)?
        }
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl<E1: ThisParser, E2: ThisParser> ThisParser for ApplyArgumentNode<E1, E2> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl<K, V, D> ThisParser for ArgumentTermNode<K, V, D>
where
    K: ThisParser,
    V: ThisParser,
    D: ThisParser,
{
    /// `key: Type = default`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, key) = input.match_fn(K::parse)?;
        let (state, value) = state.skip(ignore).match_optional(|s| {
            let (state, _) = s.match_char(':')?;
            state.skip(ignore).match_fn(V::parse)
        })?;
        let (state, default) = state.skip(ignore).match_optional(|s| {
            let (state, _) = s.match_char('=')?;
            state.skip(ignore).match_fn(D::parse)
        })?;
        state.finish(ArgumentTermNode { key, value, default });

        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

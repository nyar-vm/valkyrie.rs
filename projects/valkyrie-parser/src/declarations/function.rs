use super::*;
use valkyrie_ast::{ApplyArgumentTerm, FunctionReturnNode};

impl ThisParser for FunctionDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, head) = FunctionType::parse(input)?;
        let (state, name) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (state, generic) = state.match_optional(GenericArgumentNode::parse)?;
        let (state, args) = state.skip(ignore).match_fn(ApplyArgumentNode::parse)?;
        let (state, ret) = state.skip(ignore).match_optional(FunctionReturnNode::parse)?;
        let (finally, body) = state.skip(ignore).match_fn(FunctionBody::parse)?;
        finally.finish(FunctionDeclaration {
            r#type: head,
            namepath: name,
            modifiers: vec![],
            attributes: None,
            generic,
            arguments: args,
            r#return: ret,
            body,
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut items = vec![Lisp::keyword(self.r#type.pretty_string(144)), self.namepath.as_lisp()];
        //
        let mut args = vec![Lisp::keyword("arguments")];
        for arg in self.arguments.terms.iter() {
            args.push(arg.as_lisp());
        }
        items.push(Lisp::Any(args));
        //
        let mut body = vec![Lisp::keyword("body")];
        for term in self.body.statements.iter() {
            body.push(term.as_lisp());
        }
        items.push(Lisp::Any(body));
        //
        Lisp::Any(items)
    }
}

impl ThisParser for FunctionType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        if input.residual.starts_with("fun") {
            input.advance("fun").finish(FunctionType::Micro)
        }
        else if input.residual.starts_with("micro") {
            input.advance("micro").finish(FunctionType::Micro)
        }
        else if input.residual.starts_with("macro") {
            input.advance("macro").finish(FunctionType::Macro)
        }
        else {
            StopBecause::must_be("micro", input.start_offset)?
        }
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ThisParser for FunctionReturnNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.begin_choice().or_else(|s| s.match_str("->")).or_else(|s| s.match_str(":")).end_choice()?;
        let (state, typing) = parse_expression_node(state.skip(ignore), ExpressionContext::in_type())?;
        state.finish(FunctionReturnNode { types: typing, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for ApplyArgumentNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pattern = BracketPattern::new("(", ")");
        let (state, terms) = pattern.consume(input, ignore, ApplyArgumentTerm::parse)?;
        state.finish(ApplyArgumentNode { terms: terms.body, span: get_span(input, state) })
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
        let (state, value) = state.match_optional(|s| {
            let (state, _) = s.skip(ignore).match_char(':')?;
            state.skip(ignore).match_fn(V::parse)
        })?;
        let (state, default) = state.match_optional(|s| {
            let (state, _) = s.skip(ignore).match_char('=')?;
            state.skip(ignore).match_fn(D::parse)
        })?;
        state.finish(ArgumentTermNode { key, value, default })
    }

    fn as_lisp(&self) -> Lisp {
        let mut items = Vec::with_capacity(3);
        items.push(self.key.as_lisp());
        match &self.value {
            Some(v) => items.push(v.as_lisp()),
            None => items.push(Lisp::function("Any")),
        }
        match &self.default {
            Some(v) => items.push(v.as_lisp()),
            None => items.push(Lisp::function("null")),
        }
        Lisp::Any(items)
    }
}

impl ThisParser for ArgumentKeyNode {
    /// `mut mods name`, unconditionally catch all identifiers
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, mut default) = input.match_repeats(|s| s.skip(ignore).match_fn(IdentifierNode::parse))?;
        match default.pop() {
            Some(v) => state.finish(ArgumentKeyNode { modifiers: default, key: v }),
            None => StopBecause::must_be("identifier", input.start_offset)?,
        }
    }

    fn as_lisp(&self) -> Lisp {
        let mut items = Vec::with_capacity(self.modifiers.len() + 1);
        for modifier in self.modifiers.iter() {
            items.push(modifier.as_lisp());
        }
        items.push(self.key.as_lisp());
        Lisp::Any(items)
    }
}

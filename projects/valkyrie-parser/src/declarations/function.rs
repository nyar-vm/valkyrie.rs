use super::*;

impl ThisParser for FunctionDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, head) = FunctionType::parse(input)?;
        let (state, name) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (state, generic) = state.match_optional(GenericArgument::parse)?;
        let (state, args) = state.skip(ignore).match_fn(ApplyArgument::parse)?;
        let (state, ret) = state.skip(ignore).match_optional(FunctionReturnNode::parse)?;
        let (finally, body) = state.skip(ignore).match_fn(StatementBlock::parse)?;
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
        let mut lisp = Lisp::new(6);
        lisp += self.r#type.as_lisp();
        lisp += self.namepath.as_lisp();
        if let Some(generic) = &self.generic {
            lisp += generic.as_lisp();
        }
        lisp += self.arguments.as_lisp();
        if let Some(r#return) = &self.r#return {
            lisp += r#return.as_lisp();
        }
        lisp += self.body.as_lisp();
        lisp
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
        Lisp::keyword(self.as_str())
    }
}

impl ThisParser for FunctionReturnNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.begin_choice().choose(|s| s.match_str("->")).choose(|s| s.match_str(":")).end_choice()?;
        let (state, typing) = parse_expression_node(state.skip(ignore), ExpressionContext::in_type())?;
        state.finish(FunctionReturnNode { returns: typing, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        self.returns.as_lisp()
    }
}

impl ThisParser for FunctionEffectNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("/")?;
        let (state, typing) = parse_expression_node(state.skip(ignore), ExpressionContext::in_type())?;
        state.finish(FunctionEffectNode { effects: vec![typing], span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(self.effects.len());
        lisp += Lisp::keyword("return/effect");
        for effect in &self.effects {
            lisp += effect.as_lisp();
        }
        lisp
    }
}

impl ThisParser for ApplyArgument {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pattern = BracketPattern::new("(", ")");
        let (state, terms) = pattern.consume(input, ignore, ApplyArgumentTerm::parse)?;
        state.finish(ApplyArgument { terms: terms.body, span: get_span(input, state) })
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
        let mut lisp = Lisp::new(3);
        lisp += self.key.as_lisp();
        lisp += match &self.value {
            Some(v) => v.as_lisp(),
            None => Lisp::symbol("Any"),
        };
        lisp += match &self.default {
            Some(v) => v.as_lisp(),
            None => Lisp::keyword("null"),
        };
        lisp
    }
}

impl ThisParser for ArgumentKeyNode {
    /// `mut mods name`, unconditionally catch all identifiers
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, (mods, id)) = parse_modifiers(input)?;
        state.finish(ArgumentKeyNode { modifiers: mods, key: id })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(self.modifiers.terms.len() + 1);
        for modifier in self.modifiers.terms.iter() {
            lisp += modifier.as_lisp();
        }
        lisp += self.key.as_lisp();
        lisp
    }
}

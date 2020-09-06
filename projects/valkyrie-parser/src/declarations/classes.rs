use super::*;

impl ThisParser for ClassFieldDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, (mods, name)) = parse_modifiers(input)?;
        let (state, typing) = state.match_optional(|s| {
            let (state, _) = str(":")(s.skip(ignore))?;
            TypingExpression::parse(state.skip(ignore))
        })?;
        let (state, value) = state.match_optional(|s| {
            let (state, _) = str("=")(s.skip(ignore))?;
            ExpressionNode::parse(state.skip(ignore))
        })?;

        state.finish(ClassFieldDeclaration {
            document: Default::default(),
            modifiers: mods,
            field_name: name,
            r#type: typing.map(|s| s.as_normal()),
            default: value,
            span: get_span(input, state),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = vec![];
        for modi in &self.modifiers.terms {
            terms.push(modi.as_lisp());
        }
        terms.push(self.field_name.as_lisp());
        if let Some(typing) = &self.r#type {
            terms.push(Lisp::keyword(":"));
            terms.push(typing.as_lisp());
        }
        if let Some(value) = &self.default {
            terms.push(Lisp::keyword("="));
            terms.push(value.as_lisp());
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for ClassMethodDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for ModifiersNode {
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

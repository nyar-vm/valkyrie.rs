use super::*;

impl ThisParser for UnionDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("union")(input)?;
        let (state, name) = NamePathNode::parse(state.skip(ignore))?;
        let (state, stmt) = parse_statement_block(state.skip(ignore), union_statement)?;
        state.finish(UnionDeclaration {
            document: Default::default(),
            namepath: name,
            modifiers: ModifiersNode::default(),
            extends: None,
            implements: vec![],
            statements: stmt,
            span: get_span(input, state),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = vec![];
        terms.push(Lisp::keyword("union"));
        terms.push(self.namepath.as_lisp());
        for stmt in &self.statements.terms {
            terms.push(stmt.as_lisp());
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for VariantDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, name) = IdentifierNode::parse(input)?;
        let (state, stmt) = state.skip(ignore).match_optional(|s| parse_statement_block(s, variant_statement))?;
        let finally = state.skip(ignore).skip(parse_semi);
        finally.finish(VariantDeclaration {
            document: Default::default(),
            variant: name,
            extends: None,
            implements: vec![],
            statements: stmt.unwrap_or_default(),
            span: get_span(input, state),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = vec![];
        terms.push(self.variant.as_lisp());
        for stmt in &self.statements.terms {
            terms.push(stmt.as_lisp());
        }
        Lisp::Any(terms)
    }
}

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
            name,
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
        terms.push(self.name.as_lisp());
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

impl ThisParser for ModifiersNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

fn union_statement(input: ParseState) -> ParseResult<StatementNode> {
    let (state, ty) = input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| DocumentationNode::parse(s).map_inner(Into::into))
        .or_else(|s| VariantDeclaration::parse(s).map_inner(Into::into))
        .end_choice()?;
    state.finish(StatementNode { r#type: ty, end_semicolon: true, span: get_span(input, state) })
}

fn variant_statement(input: ParseState) -> ParseResult<StatementNode> {
    let (state, ty) = input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| DocumentationNode::parse(s).map_inner(Into::into))
        .or_else(|s| ClassFieldDeclaration::parse(s).map_inner(Into::into))
        .end_choice()?;
    state.finish(StatementNode { r#type: ty, end_semicolon: true, span: get_span(input, state) })
}

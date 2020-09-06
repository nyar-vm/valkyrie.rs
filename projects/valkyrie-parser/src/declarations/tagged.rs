use super::*;

impl ThisParser for TaggedDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("tagged")(input)?;
        let (state, name) = NamePathNode::parse(state.skip(ignore))?;
        let (state, stmt) = parse_statement_block(state.skip(ignore), union_statement)?;
        state.finish(TaggedDeclaration {
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
        terms.push(Lisp::keyword("tagged"));
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

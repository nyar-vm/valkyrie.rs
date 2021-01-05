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

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(10);
        lisp += Lisp::keyword("tagged");
        lisp += self.namepath.lispify();
        for stmt in &self.statements.terms {
            lisp += stmt.lispify();
        }
        lisp
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

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(self.statements.terms.len() + 2);
        lisp += self.variant.lispify();
        for stmt in &self.statements.terms {
            lisp += stmt.lispify();
        }
        lisp
    }
}

fn union_statement(input: ParseState) -> ParseResult<StatementNode> {
    let (state, ty) = input
        .skip(ignore)
        .begin_choice()
        .choose(|s| DocumentationNode::parse(s).map_into())
        .choose(|s| VariantDeclaration::parse(s).map_into())
        .end_choice()?;
    state.finish(StatementNode { r#type: ty, end_semicolon: true, span: get_span(input, state) })
}

fn variant_statement(input: ParseState) -> ParseResult<StatementNode> {
    let (state, ty) = input
        .skip(ignore)
        .begin_choice()
        .choose(|s| DocumentationNode::parse(s).map_into())
        .choose(|s| ClassFieldDeclaration::parse(s).map_into())
        .end_choice()?;
    state.finish(StatementNode { r#type: ty, end_semicolon: true, span: get_span(input, state) })
}

use super::*;

impl ThisParser for ClassDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("class")?;
        let (state, namepath) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (state, stmt) = parse_statement_block(state.skip(ignore), class_statements)?;
        state.finish(ClassDeclaration {
            kind: ClassKind::Class,
            namepath,
            modifiers: vec![],
            extends: None,
            implements: vec![],
            body: stmt,
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("class");
        lisp += self.namepath.as_lisp();
        lisp
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
        let finally = state.skip(ignore).skip(parse_semi);
        finally.finish(ClassFieldDeclaration {
            document: Default::default(),
            modifiers: mods,
            field_name: name,
            r#type: typing.map(|s| s.as_normal()),
            default: value,
            span: get_span(input, state),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(10);
        lisp += Lisp::keyword("class/field");
        lisp += self.field_name.as_lisp();
        lisp += self.modifiers.as_lisp();
        if let Some(typing) = &self.r#type {
            lisp += Lisp::keyword(":");
            lisp += typing.as_lisp();
        }
        if let Some(value) = &self.default {
            lisp += Lisp::keyword("=");
            lisp += value.as_lisp();
        }
        lisp
    }
}

impl ThisParser for ClassMethodDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, (mods, id)) = parse_modifiers(input)?;
        state.finish(Self { modifiers: mods, method_name: id })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("class/method");
        lisp += self.method_name.as_lisp();
        lisp += self.modifiers.as_lisp();
        lisp
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

fn class_statements(input: ParseState) -> ParseResult<StatementNode> {
    let (state, ty) = input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| DocumentationNode::parse(s).map_into())
        .or_else(|s| ClassFieldDeclaration::parse(s).map_into())
        .or_else(|s| AnnotationList::parse(s).map_into())
        .or_else(|s| AnnotationNode::parse(s).map_into())
        .end_choice()?;
    state.finish(StatementNode { r#type: ty, end_semicolon: true, span: get_span(input, state) })
}

use super::*;
use lispify::Lispify;

impl ThisParser for ClassDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("class")?;
        let (state, namepath) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (state, generic) = state.match_optional(GenericArgument::parse)?;
        let (finally, stmt) = parse_statement_block(state.skip(ignore), class_statements)?;
        finally.finish(ClassDeclaration {
            kind: ClassKind::Class,
            name: namepath,
            generic,
            modifiers: ModifiersNode::default(),
            auto_traits: vec![],
            body: stmt,
            base_classes: None,
            span: get_span(input, finally),
        })
    }
}

impl ThisParser for ClassKind {
    fn parse(input: ParseState) -> ParseResult<Self> {
        if input.residual.starts_with("class") {
            input.advance("class").finish(ClassKind::Class)
        }
        else if input.residual.starts_with("structure") {
            input.advance("structure").finish(ClassKind::Structure)
        }
        else {
            StopBecause::must_be("KW_CLASS", input.start_offset)?
        }
    }

    fn as_lisp(&self) -> Lisp {
        let define = match self {
            Self::Class => "define/class",
            Self::Structure => "define/structure",
        };
        Lisp::keyword(define)
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
        let (state, generic) = state.match_optional(GenericArgument::parse)?;
        let (state, params) = state.skip(ignore).match_fn(ApplyArgument::parse)?;
        let (state, return_type) = state.skip(ignore).match_optional(FunctionReturnNode::parse)?;
        let (state, effect_type) = state.skip(ignore).match_optional(FunctionEffectNode::parse)?;
        let (finally, body) = state.skip(ignore).match_optional(StatementBlock::parse)?;
        finally.finish(Self {
            document: Default::default(),
            modifiers: mods,
            method_name: id,
            generic,
            arguments: params,
            return_type,
            effect_type,
            body,
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("class/method");
        lisp += self.method_name.as_lisp();
        lisp += self.modifiers.as_lisp();
        if let Some(generic) = &self.generic {
            if !generic.terms.is_empty() {
                lisp += generic.as_lisp();
            }
        }
        lisp += self.arguments.as_lisp();
        match &self.return_type {
            Some(ty) => {
                lisp += Lisp::keyword("return/type") + ty.as_lisp();
            }
            None => {
                lisp += Lisp::keyword("return/type") + Lisp::symbol("Unit");
            }
        }
        match &self.effect_type {
            Some(ty) => lisp += ty.as_lisp(),
            None => {
                lisp += Lisp::keyword("return/type") + Lisp::symbol("Pure");
            }
        }
        if let Some(body) = &self.body {
            for item in &body.terms {
                lisp += item.as_lisp();
            }
        }
        lisp
    }
}

fn class_statements(input: ParseState) -> ParseResult<StatementNode> {
    let (state, ty) = input
        .skip(ignore)
        .begin_choice()
        .choose(|s| DocumentationNode::parse(s).map_into())
        .choose(|s| ClassMethodDeclaration::parse(s).map_into())
        .choose(|s| ClassFieldDeclaration::parse(s).map_into())
        .choose(|s| AnnotationList::parse(s).map_into())
        .choose(|s| AnnotationNode::parse(s).map_into())
        .end_choice()?;
    let finally = state.skip(ignore).skip(parse_semi);
    finally.finish(StatementNode { r#type: ty, end_semicolon: true, span: get_span(input, finally) })
}

#[test]
fn test_statement() {
    let raw = ParseState::new("method()");
    let apply = class_statements(raw).unwrap();
    println!("{:#?}", apply);
}

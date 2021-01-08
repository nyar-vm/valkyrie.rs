use super::*;
use valkyrie_ast::{FunctionDeclaration, FunctionReturnNode, FunctionType, IfStatement};

mod atomic;

impl ParseTreeVisitorCompat<'_> for ValkyrieProgramParser {
    type Node = ValkyrieAntlrParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        unreachable!()
    }
    fn visit(&mut self, node: &<Self::Node as ParserNodeType<'_>>::Type) -> Self::Return {
        node.accept_dyn(self);
    }
}

/// Convert weakly typed ast to strongly typed ast
impl ValkyrieAntlrVisitor<'_> for ValkyrieProgramParser {
    fn visit_program(&mut self, ctx: &ProgramContext<'_>) {
        for node in ctx.top_statement_all() {
            if let Some(s) = StatementNode::take_one(&*node) {
                self.statements.push(s)
            }
        }
    }
}

impl<'i> Extractor<Top_statementContextAll<'i>> for StatementNode {
    fn take_one(node: &Top_statementContextAll<'i>) -> Option<Self> {
        let body: StatementType = match node {
            Top_statementContextAll::SClassContext(s) => ClassDeclaration::take(s.define_class())?.into(),
            Top_statementContextAll::SExtendsContext(_) => {
                todo!()
            }
            Top_statementContextAll::STraitContext(_) => {
                todo!()
            }
            Top_statementContextAll::SNamespaceContext(s) => NamespaceDeclaration::take(s.define_namespace())?.into(),
            Top_statementContextAll::SExtensionContext(_) => {
                todo!()
            }
            Top_statementContextAll::SFunctionContext(s) => FunctionDeclaration::take(s.define_function())?.into(),
            Top_statementContextAll::SImportContext(_) => {
                todo!()
            }
            Top_statementContextAll::SFlagsContext(s) => FlagsDeclaration::take(s.define_bitflags())?.into(),
            Top_statementContextAll::SUnionContext(s) => UnionDeclaration::take(s.define_union())?.into(),
            Top_statementContextAll::S1Context(rest) => StatementType::take(rest.function_statement())?,
            Top_statementContextAll::Error(_) => {
                todo!()
            }
        };
        Some(Self { r#type: body, end_semicolon: false })
    }
}

impl<'i> Extractor<Function_statementContextAll<'i>> for StatementType {
    fn take_one(node: &Function_statementContextAll<'i>) -> Option<Self> {
        let body: StatementType = match node {
            Function_statementContextAll::SlambdaContext(_) => {
                todo!()
            }
            Function_statementContextAll::SLoopContext(_) => {
                todo!()
            }
            Function_statementContextAll::STypeContext(_) => {
                todo!()
            }
            Function_statementContextAll::SIfLetContext(_) => {
                todo!()
            }
            Function_statementContextAll::SLetContext(_) => {
                todo!()
            }
            Function_statementContextAll::S2Context(rest) => ExpressionNode::take(rest.expression_root())?.into(),
            Function_statementContextAll::Error(_) => {
                todo!()
            }
        };
        Some(body)
    }
}

impl<'i> Extractor<Define_functionContextAll<'i>> for FunctionDeclaration {
    fn take_one(node: &Define_functionContextAll<'i>) -> Option<Self> {
        Some(Self {
            r#type: FunctionType::Macro,
            namepath: NamePathNode { names: vec![] },
            modifiers: vec![],
            attributes: None,
            generic: None,
            arguments: Default::default(),
            r#return: None,
            body: Default::default(),
        })
    }
}
impl<'i> Extractor<Expression_rootContext<'i>> for ExpressionNode {
    fn take_one(node: &Expression_rootContextAll<'i>) -> Option<Self> {
        let body = node.expression()?;
        let node = ExpressionType::take_one(&*body)?;
        Some(Self { type_level: false, body: node, span: Default::default() })
    }
}
impl<'i> Extractor<ExpressionContextAll<'i>> for ExpressionType {
    fn take_one(node: &ExpressionContextAll<'i>) -> Option<Self> {
        let body = match node {
            ExpressionContextAll::EPrefixContext(prefix) => PrefixNode::take_one(prefix)?.into(),
            ExpressionContextAll::ESuffixContext(suffix) => PostfixNode::take_one(suffix)?.into(),
            ExpressionContextAll::EPowContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::EPlusContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::EMulContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::EIsAContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::ECompareContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::EPipeContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::EInContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::ELogicContext(infix) => InfixNode::take_one(infix)?.into(),
            ExpressionContextAll::EGroupContext(group) => ExpressionType::take(group.expression())?,
            ExpressionContextAll::ESliceContext(_) => {
                todo!()
            }
            ExpressionContextAll::EOffsetContext(_) => {
                todo!()
            }
            ExpressionContextAll::EAsContext(_) => {
                todo!()
            }
            ExpressionContextAll::EAssignContext(_) => {
                todo!()
            }
            ExpressionContextAll::ELambdaContext(_) => {
                todo!()
            }
            ExpressionContextAll::EDotContext(_) => {
                todo!()
            }
            ExpressionContextAll::EOrElseContext(_) => {
                todo!()
            }
            ExpressionContextAll::EGenericContext(_) => {
                todo!()
            }
            ExpressionContextAll::EControlContext(_) => {
                todo!()
            }
            ExpressionContextAll::EUntilContext(_) => {
                todo!()
            }
            ExpressionContextAll::EDotMatchContext(_) => {
                todo!()
            }
            ExpressionContextAll::EDotFunctionContext(_) => {
                todo!()
            }
            ExpressionContextAll::E1Context(rest) => ExpressionType::take(rest.term_with_follow())?,
            ExpressionContextAll::Error(_) => {
                todo!()
            }
        };
        Some(body)
    }
}
impl<'i> Extractor<Inline_expressionContextAll<'i>> for ExpressionType {
    fn take_one(node: &Inline_expressionContextAll<'i>) -> Option<Self> {
        let body = match node {
            Inline_expressionContextAll::ILogicContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::IDotContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::IRangeContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::IMulContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::IPlusContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::ICompareContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::IAsContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::IPrefixContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::IIsAContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::E3Context(_) => {
                todo!()
            }
            Inline_expressionContextAll::ISliceContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::Error(_) => {
                todo!()
            }
        };
        Some(body)
    }
}
impl<'i> Extractor<Type_expressionContextAll<'i>> for ExpressionType {
    fn take_one(node: &Type_expressionContextAll<'i>) -> Option<Self> {
        let body = match node {
            Type_expressionContextAll::TGenericContext(_) => {
                todo!()
            }
            Type_expressionContextAll::TPatternContext(_) => {
                todo!()
            }
            Type_expressionContextAll::TTupleContext(_) => {
                todo!()
            }
            Type_expressionContextAll::TAddContext(_) => {
                todo!()
            }
            Type_expressionContextAll::TArrowsContext(_) => {
                todo!()
            }
            Type_expressionContextAll::E4Context(_) => {
                todo!()
            }
            Type_expressionContextAll::Error(_) => {
                todo!()
            }
        };
        Some(body)
    }
}
impl<'i> Extractor<Term_with_followContextAll<'i>> for ExpressionType {
    fn take_one(node: &Term_with_followContextAll<'i>) -> Option<Self> {
        let body = match node {
            Term_with_followContextAll::SIfContext(s) => IfStatement::take(s.if_statement())?.into(),
            Term_with_followContextAll::ETryContext(_) => {
                todo!()
            }
            Term_with_followContextAll::EFunctionContext(_) => {
                todo!()
            }
            Term_with_followContextAll::EDefineContext(_) => {
                todo!()
            }
            Term_with_followContextAll::EObjectContext(_) => {
                todo!()
            }
            Term_with_followContextAll::ETupleContext(_) => {
                todo!()
            }
            Term_with_followContextAll::ERangeContext(_) => {
                todo!()
            }
            Term_with_followContextAll::EMatchContext(_) => {
                todo!()
            }
            Term_with_followContextAll::ENewContext(_) => {
                todo!()
            }
            Term_with_followContextAll::EMacroContext(_) => {
                todo!()
            }
            Term_with_followContextAll::E2Context(rest) => ExpressionType::take(rest.atomic())?.into(),
            Term_with_followContextAll::Error(_) => {
                todo!()
            }
        };
        Some(body)
    }
}

impl<'i> Extractor<If_statementContextAll<'i>> for IfStatement {
    fn take_one(node: &If_statementContextAll<'i>) -> Option<Self> {
        // node.then_block

        // let body = node.expression()?;
        // let node = ExpressionType::take_one(&*body)?;
        Some(Self { branches: vec![], else_body: None, span: Default::default() })
    }
}

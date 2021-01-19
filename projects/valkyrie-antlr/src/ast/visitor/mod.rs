use super::*;
use valkyrie_ast::{ExtendsStatement, FunctionDeclaration, FunctionType, GuardPattern, GuardStatement, IfStatement};

mod atomic;
mod looping;

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
        for node in ctx.get_children() {
            if let Some(s) = node.downcast_ref::<Define_namespaceContextAll>().and_then(NamespaceDeclaration::take_one) {
                self.statements.push(s.into());
                continue;
            }
            if let Some(s) = node.downcast_ref::<Define_classContextAll>().and_then(ClassDeclaration::take_one) {
                self.statements.push(s.into());
                continue;
            }
            if let Some(s) = node.downcast_ref::<Define_bitflagsContextAll>().and_then(FlagsDeclaration::take_one) {
                self.statements.push(s.into());
                continue;
            }
            if let Some(s) = node.downcast_ref::<Define_unionContextAll>().and_then(UnionDeclaration::take_one) {
                self.statements.push(s.into());
                continue;
            }
            if let Some(s) = node.downcast_ref::<Define_extendsContextAll>().and_then(ExtendsStatement::take_one) {
                self.statements.push(s.into());
                continue;
            }
            if let Some(s) = node.downcast_ref::<Define_functionContextAll>().and_then(FunctionDeclaration::take_one) {
                self.statements.push(s.into());
                continue;
            }
            if let Some(s) = node.downcast_ref::<Expression_rootContextAll>().and_then(ExpressionNode::take_one) {
                self.statements.push(s.into());
                continue;
            }
        }
        // Function_statementContextAll::SLoopContext(v) => StatementType::take(v.loop_statement())?.into(),
        // Function_statementContextAll::SIfLetContext(s) => GuardStatement::take(s.guard_statement())?.into(),
    }
}

impl<'i> Extractor<Define_extendsContextAll<'i>> for ExtendsStatement {
    fn take_one(node: &Define_extendsContextAll<'i>) -> Option<Self> {
        Some(Self { fields: vec![], methods: vec![] })
    }
}

impl<'i> Extractor<Guard_statementContextAll<'i>> for GuardStatement {
    fn take_one(node: &Guard_statementContextAll<'i>) -> Option<Self> {
        let negative = node.KW_NOT().is_some();
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        let place = ExpressionNode {
            type_level: false,
            body: ExpressionType::Number(Box::new(NumberLiteralNode::new("0", 0..0))),
            span: span.clone(),
        };
        Some(Self { negative, condition: place.clone(), main_body: GuardPattern::Expression(place), span })
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
            ExpressionContextAll::EPowContext(infix) => {
                let operator = OperatorNode::take(infix.infix_pow())?;
                let lhs = ExpressionType::take(infix.lhs.clone())?;
                let rhs = ExpressionType::take(infix.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            ExpressionContextAll::EPlusContext(infix) => {
                let operator = OperatorNode::take(infix.op_plus())?;
                let lhs = ExpressionType::take(infix.lhs.clone())?;
                let rhs = ExpressionType::take(infix.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            ExpressionContextAll::EMulContext(infix) => {
                let operator = OperatorNode::take(infix.op_multiple())?;
                let lhs = ExpressionType::take(infix.lhs.clone())?;
                let rhs = ExpressionType::take(infix.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            ExpressionContextAll::EIsAContext(infix) => {
                let operator = OperatorNode::take(infix.infix_is())?;
                let lhs = ExpressionType::take(infix.lhs.clone())?;
                let rhs = ExpressionType::take(infix.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            ExpressionContextAll::ECompareContext(infix) => {
                let operator = OperatorNode::take(infix.op_compare())?;
                let lhs = ExpressionType::take(infix.lhs.clone())?;
                let rhs = ExpressionType::take(infix.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            ExpressionContextAll::EPipeContext(infix) => {
                let operator = OperatorNode::take(infix.op_pipeline())?;
                let lhs = ExpressionType::take(infix.lhs.clone())?;
                let rhs = ExpressionType::take(infix.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            ExpressionContextAll::EInContext(infix) => {
                let operator = OperatorNode::take(infix.infix_in())?;
                let lhs = ExpressionType::take(infix.lhs.clone())?;
                let rhs = ExpressionType::take(infix.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            ExpressionContextAll::ELogicContext(infix) => {
                let operator = OperatorNode::take(infix.op_logic())?;
                let lhs = ExpressionType::take(infix.lhs.clone())?;
                let rhs = ExpressionType::take(infix.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            ExpressionContextAll::EGroupContext(group) => ExpressionType::take(group.expression())?,
            ExpressionContextAll::ESliceContext(slice) => {
                // SubscriptSliceNode::take(slice.slice_call())
                todo!()
            }
            ExpressionContextAll::EAsContext(_) => {
                todo!()
            }
            ExpressionContextAll::EAssignContext(infix) => {
                println!("EAssignContext: {infix:?}");
                return None;
            }
            ExpressionContextAll::ELambdaContext(_) => {
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
            ExpressionContextAll::EIfContext(v) => IfStatement::take(v.if_statement())?.into(),
            ExpressionContextAll::ETryContext(_) => {
                todo!()
            }
            ExpressionContextAll::EFloorContext(_) => {
                todo!()
            }
            ExpressionContextAll::ECeilingContext(_) => {
                todo!()
            }
            ExpressionContextAll::EObjectContext(_) => {
                todo!()
            }
            ExpressionContextAll::ETupleContext(v) => v,
            ExpressionContextAll::ERangeContext(_) => {
                todo!()
            }

            ExpressionContextAll::EFunctionContext(_) => {
                todo!()
            }
            ExpressionContextAll::EClosureContext(_) => {
                todo!()
            }
            ExpressionContextAll::EMatchContext(_) => {
                todo!()
            }
            ExpressionContextAll::EMacroContext(_) => {
                todo!()
            }
            ExpressionContextAll::EDefineContext(_) => {
                todo!()
            }
            ExpressionContextAll::EMapContext(_) => {
                todo!()
            }
            ExpressionContextAll::ENewContext(_) => {
                todo!()
            }
            ExpressionContextAll::EAtomContext(atom) => ExpressionType::take(atom.atomic())?,
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
            Inline_expressionContextAll::ILogicContext(inline) => {
                let operator = OperatorNode::take(inline.op_logic())?;
                let lhs = ExpressionType::take(inline.rhs.clone())?;
                let rhs = ExpressionType::take(inline.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            Inline_expressionContextAll::IRangeContext(inline) => {
                todo!()
            }
            Inline_expressionContextAll::IMulContext(inline) => {
                let operator = OperatorNode::take(inline.op_multiple())?;
                let lhs = ExpressionType::take(inline.rhs.clone())?;
                let rhs = ExpressionType::take(inline.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            Inline_expressionContextAll::IPlusContext(inline) => {
                let operator = OperatorNode::take(inline.op_plus())?;
                let lhs = ExpressionType::take(inline.rhs.clone())?;
                let rhs = ExpressionType::take(inline.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            Inline_expressionContextAll::ICompareContext(inline) => {
                let operator = OperatorNode::take(inline.op_compare())?;
                let lhs = ExpressionType::take(inline.rhs.clone())?;
                let rhs = ExpressionType::take(inline.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }
            Inline_expressionContextAll::IAsContext(inline) => {
                todo!()
            }
            Inline_expressionContextAll::IPrefixContext(inline) => {
                todo!()
            }
            Inline_expressionContextAll::IIsContext(inline) => {
                let operator = OperatorNode::take(inline.infix_is())?;
                let lhs = ExpressionType::take(inline.rhs.clone())?;
                let rhs = ExpressionType::take(inline.rhs.clone())?;
                InfixNode { infix: operator, lhs, rhs }.into()
            }

            Inline_expressionContextAll::ISliceContext(_) => {
                todo!()
            }

            Inline_expressionContextAll::IGenericContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::IMapContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::ITupleContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::IAtomContext(atom) => ExpressionType::take(atom.atomic())?,
            Inline_expressionContextAll::Error(_) => {
                todo!()
            }
            Inline_expressionContextAll::IFunctionContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::IGroupContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::IFloorContext(_) => {
                todo!()
            }
            Inline_expressionContextAll::ICeilingContext(_) => {
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
            Type_expressionContextAll::TAtomContext(atom) => ExpressionType::take(atom.atomic())?,
            Type_expressionContextAll::TPrefixContext(_) => {
                todo!()
            }
            Type_expressionContextAll::TOptionalContext(_) => {
                todo!()
            }
            Type_expressionContextAll::Error(_) => {
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

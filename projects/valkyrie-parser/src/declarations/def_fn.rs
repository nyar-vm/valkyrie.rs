use super::*;

impl crate::DefineFunctionNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<FunctionDeclaration> {
        let annotations = self.annotation_head.annotations(ctx)?;
        let parameters = self.function_middle.parameters(ctx)?;
        let generic = self.function_middle.generics(ctx)?;
        let returns = self.function_middle.returns(ctx)?;
        let body = self.continuation.build(ctx)?;
        Ok(FunctionDeclaration {
            name: self.namepath.build(ctx),
            kind: self.kw_function.build(),
            annotations,
            generics: generic,
            parameters,
            returns,
            body,
        })
    }
}

impl crate::KwFunctionNode {
    pub(crate) fn build(&self) -> FunctionKind {
        match self {
            Self::Micro => FunctionKind::Micro,
            Self::Macro => FunctionKind::Macro,
        }
    }
}

impl crate::FunctionMiddleNode {
    pub(crate) fn returns(&self, ctx: &mut ProgramState) -> Result<FunctionReturnNode> {
        let typing = match &self.type_return {
            Some(s) => Some(s.type_expression.build(ctx)?),
            None => None,
        };
        Ok(FunctionReturnNode { typing, effect: vec![] })
    }
    pub(crate) fn parameters(&self, ctx: &mut ProgramState) -> Result<ParametersList> {
        let mut terms = vec![];
        for term in &self.function_parameters.parameter_item {
            match term.build(ctx) {
                Ok(s) => terms.push(s),
                Err(e) => ctx.add_error(e),
            }
        }
        Ok(ParametersList { kind: ParameterKind::Expression, terms })
    }
    pub(crate) fn generics(&self, ctx: &mut ProgramState) -> Result<ParametersList> {
        let mut terms = vec![];
        match &self.define_generic {
            Some(s) => {
                for term in &s.generic_parameter.generic_parameter_pair {
                    match term.build(ctx) {
                        Ok(s) => terms.push(s),
                        Err(e) => ctx.add_error(e),
                    }
                }
            }
            None => {}
        }
        Ok(ParametersList { kind: ParameterKind::Generic, terms })
    }
}

impl crate::GenericParameterNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ParametersList> {
        let mut terms = vec![];
        for term in &self.generic_parameter_pair {
            match term.build(ctx) {
                Ok(s) => terms.push(s),
                Err(e) => ctx.add_error(e),
            }
        }
        Ok(ParametersList { kind: ParameterKind::Generic, terms })
    }
}

impl crate::DefineGenericNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ParametersList> {
        let mut terms = vec![];
        for term in &self.generic_parameter.generic_parameter_pair {
            match term.build(ctx) {
                Ok(s) => terms.push(s),
                Err(e) => ctx.add_error(e),
            }
        }
        Ok(ParametersList { kind: ParameterKind::Generic, terms })
    }
}

impl crate::GenericParameterPairNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ParameterTerm> {
        let key = self.identifier.build(ctx);
        Ok(ParameterTerm::Single {
            annotations: Default::default(),
            key,
            bound: self.build_bound(ctx),
            default: self.build_default(ctx),
        })
    }
    fn build_bound(&self, ctx: &mut ProgramState) -> Option<ExpressionKind> {
        match self.bound.as_ref()?.build(ctx) {
            Ok(o) => Some(o),
            Err(e) => {
                ctx.add_error(e);
                None
            }
        }
    }
    fn build_default(&self, ctx: &mut ProgramState) -> Option<ExpressionKind> {
        match self.default.as_ref()?.build(ctx) {
            Ok(o) => Some(o),
            Err(e) => {
                ctx.add_error(e);
                None
            }
        }
    }
}
impl crate::ParameterItemNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ParameterTerm> {
        let value = match self {
            Self::LMark => ParameterTerm::LMark,
            Self::OmitDict => ParameterTerm::LMark,
            Self::OmitList => ParameterTerm::LMark,
            Self::ParameterPair(v) => v.build(ctx)?,
            Self::RMark => ParameterTerm::RMark,
        };
        Ok(value)
    }
}

impl crate::ParameterPairNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ParameterTerm> {
        let key = self.identifier.build(ctx);
        Ok(ParameterTerm::Single {
            annotations: Default::default(),
            key,
            bound: self.build_bound(ctx),
            default: self.build_default(ctx),
        })
    }
    fn build_bound(&self, ctx: &mut ProgramState) -> Option<ExpressionKind> {
        match self.type_hint.as_ref()?.build(ctx) {
            Ok(o) => Some(o),
            Err(e) => {
                ctx.add_error(e);
                None
            }
        }
    }
    fn build_default(&self, ctx: &mut ProgramState) -> Option<ExpressionKind> {
        match self.parameter_default.as_ref()?.main_expression.build(ctx) {
            Ok(o) => Some(o),
            Err(e) => {
                ctx.add_error(e);
                None
            }
        }
    }
}

impl crate::ContinuationNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<StatementBlock> {
        let mut terms = vec![];
        for term in &self.main_statement {
            match term.build(ctx) {
                Ok(s) => terms.extend(s),
                Err(e) => ctx.add_error(e),
            }
        }
        Ok(StatementBlock { terms, span: self.span.clone() })
    }
}
impl crate::TypeHintNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ExpressionKind> {
        self.type_expression.build(ctx)
    }
}

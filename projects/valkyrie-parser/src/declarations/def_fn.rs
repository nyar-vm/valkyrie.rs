use super::*;
use crate::utils::{build_parameter_default, build_type_hint};

impl crate::DefineFunctionNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<FunctionDeclaration> {
        let annotations = self.annotation_head.annotations(ctx)?;
        let returns = self.function_middle.returns(ctx)?;
        Ok(FunctionDeclaration {
            name: self.namepath.build(ctx),
            kind: self.kw_function.build(),
            annotations,
            generics: self.function_middle.generics(ctx),
            parameters: self.function_middle.parameters(ctx),
            returns,
            body: self.continuation.build(ctx),
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
        let effect = match &self.type_effect {
            Some(_) => {
                vec![]
            }
            None => {
                vec![]
            }
        };

        Ok(FunctionReturnNode { typing, effect: effect })
    }
    pub(crate) fn parameters(&self, ctx: &mut ProgramState) -> ParametersList {
        self.function_parameters.build(ctx)
    }
    pub(crate) fn generics(&self, ctx: &mut ProgramState) -> ParametersList {
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
        ParametersList { kind: ParameterKind::Generic, terms }
    }
}

impl FunctionParametersNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> ParametersList {
        let mut list = ParametersList::new(self.parameter_item.len(), ParameterKind::Expression);
        for term in &self.parameter_item {
            match term.build(ctx) {
                Ok(s) => list.terms.push(s),
                Err(e) => ctx.add_error(e),
            }
        }
        list
    }
}

impl crate::GenericParameterNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> ParametersList {
        let mut list = ParametersList::new(self.generic_parameter_pair.len(), ParameterKind::Generic);
        for term in &self.generic_parameter_pair {
            match term.build(ctx) {
                Ok(s) => list.terms.push(s),
                Err(e) => ctx.add_error(e),
            }
        }
        list
    }
}

impl crate::DefineGenericNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> ParametersList {
        self.generic_parameter.build(ctx)
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
            annotations: self.annotations(ctx),
            key,
            bound: build_type_hint(&self.type_hint, ctx),
            default: build_parameter_default(&self.parameter_default, ctx),
        })
    }
    fn annotations(&self, ctx: &mut ProgramState) -> AnnotationNode {
        let mut out = AnnotationNode::default();
        out.modifiers = build_modifier_ahead(&self.modifier_ahead, ctx);
        out
    }
}

impl crate::ContinuationNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> StatementBlock {
        let mut out = StatementBlock::new(self.main_statement.len(), &self.span);
        for term in &self.main_statement {
            match term.build(ctx) {
                Ok(s) => out.terms.extend(s),
                Err(e) => ctx.add_error(e),
            }
        }
        out
    }
}
impl crate::TypeHintNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ExpressionKind> {
        self.type_expression.build(ctx)
    }
}

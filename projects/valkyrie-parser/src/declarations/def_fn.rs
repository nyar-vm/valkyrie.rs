use super::*;
use crate::{
    DefineGenericNode, GenericParameterPairNode, ParameterDefaultNode, ParameterItemNode, ParameterPairNode, TypeReturnNode,
};

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
    pub fn returns(&self, ctx: &mut ProgramState) -> Result<FunctionReturnNode> {
        let typing = match &self.type_return {
            Some(s) => Some(s.type_expression.build(ctx)?),
            None => None,
        };
        Ok(FunctionReturnNode { typing, effect: vec![] })
    }
    pub fn parameters(&self, ctx: &mut ProgramState) -> Result<ParametersList> {
        let mut terms = vec![];
        for term in &self.function_parameters.parameter_item {
            match term.build(ctx) {
                Ok(s) => terms.push(s),
                Err(e) => ctx.add_error(e),
            }
        }
        Ok(ParametersList { kind: ParameterKind::Expression, terms })
    }
    pub fn generics(&self, ctx: &mut ProgramState) -> Result<ParametersList> {
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

impl GenericParameterPairNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ParameterTerm> {
        let key = self.identifier.build(ctx);
        let bound = match &self.bound {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        let default = match &self.default {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        Ok(ParameterTerm::Single { annotations: Default::default(), key, bound, default })
    }
}
impl ParameterItemNode {
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

impl ParameterPairNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ParameterTerm> {
        let key = self.identifier.build(ctx);
        let bound = match &self.type_hint {
            Some(s) => Some(s.type_expression.build(ctx)?),
            None => None,
        };
        let default = match &self.parameter_default {
            Some(s) => Some(s.main_expression.build(ctx)?),
            None => None,
        };
        Ok(ParameterTerm::Single { annotations: Default::default(), key, bound, default })
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

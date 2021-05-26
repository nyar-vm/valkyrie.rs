use super::*;
use crate::{
    DefineGenericNode, GenericParameterPairNode, ParameterDefaultNode, ParameterItemNode, ParameterPairNode, TypeReturnNode,
};

impl crate::DefineFunctionNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<FunctionDeclaration> {
        let mut errors = vec![];
        let annotations = self.annotation_head.annotations(ctx).recover(&mut errors)?;
        let parameters = self.function_middle.parameters(ctx).recover(&mut errors)?;
        let generic = self.function_middle.generics(ctx).recover(&mut errors)?;
        let returns = self.function_middle.returns(ctx).recover(&mut errors)?;
        let body = self.continuation.build(ctx).recover(&mut errors)?;
        Success {
            value: FunctionDeclaration {
                name: self.namepath.build(ctx),
                kind: self.kw_function.build(),
                annotations,
                generics: generic,
                parameters,
                returns,
                body,
            },
            diagnostics: errors,
        }
    }
}

impl crate::KwFunctionNode {
    pub fn build(&self) -> FunctionKind {
        match self {
            Self::Micro => FunctionKind::Micro,
            Self::Macro => FunctionKind::Macro,
        }
    }
}

impl crate::FunctionMiddleNode {
    pub fn returns(&self, ctx: &mut ProgramState) -> Validation<FunctionReturnNode> {
        let mut errors = vec![];
        let typing = match &self.type_return {
            Some(s) => Some(s.type_expression.build(ctx)?),
            None => None,
        };
        Success { value: FunctionReturnNode { typing, effect: vec![] }, diagnostics: errors }
    }
    pub fn parameters(&self, ctx: &mut ProgramState) -> Validation<ParametersList> {
        let mut errors = vec![];
        let mut terms = vec![];
        for term in &self.function_parameters.parameter_item {
            term.build(ctx).append(&mut terms, &mut errors)
        }
        Success { value: ParametersList { kind: ParameterKind::Expression, terms }, diagnostics: errors }
    }
    pub fn generics(&self, ctx: &mut ProgramState) -> Validation<ParametersList> {
        let mut errors = vec![];
        let mut terms = vec![];
        match &self.define_generic {
            Some(s) => {
                for term in &s.generic_parameter.generic_parameter_pair {
                    term.build(ctx).append(&mut terms, &mut errors)
                }
            }
            None => {}
        }
        Success { value: ParametersList { kind: ParameterKind::Generic, terms }, diagnostics: errors }
    }
}

impl DefineGenericNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ParametersList> {
        let mut diagnostics = vec![];
        let mut terms = vec![];
        for term in &self.generic_parameter.generic_parameter_pair {
            term.build(ctx).append(&mut terms, &mut diagnostics)
        }
        Success { value: ParametersList { kind: ParameterKind::Generic, terms }, diagnostics }
    }
}

impl GenericParameterPairNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ParameterTerm> {
        let mut diagnostics = vec![];
        let key = self.identifier.build(ctx);
        let bound = match &self.bound {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        let default = match &self.default {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        Success { value: ParameterTerm::Single { annotations: Default::default(), key, bound, default }, diagnostics }
    }
}
impl ParameterItemNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ParameterTerm> {
        let mut diagnostics = vec![];
        let value = match self {
            Self::LMark => ParameterTerm::LMark,
            Self::OmitDict => ParameterTerm::LMark,
            Self::OmitList => ParameterTerm::LMark,
            Self::ParameterPair(v) => v.build(ctx)?,
            Self::RMark => ParameterTerm::RMark,
        };
        Success { value, diagnostics }
    }
}

impl ParameterPairNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ParameterTerm> {
        let mut diagnostics = vec![];
        let key = self.identifier.build(ctx);
        let bound = match &self.type_hint {
            Some(s) => Some(s.type_expression.build(ctx)?),
            None => None,
        };
        let default = match &self.parameter_default {
            Some(s) => Some(s.main_expression.build(ctx)?),
            None => None,
        };
        Success { value: ParameterTerm::Single { annotations: Default::default(), key, bound, default }, diagnostics }
    }
}

impl crate::ContinuationNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<StatementBlock> {
        let mut diagnostics = vec![];
        let mut terms = vec![];
        for term in &self.main_statement {
            term.build(ctx).append(&mut terms, &mut diagnostics)
        }
        Success { value: StatementBlock { terms, span: self.span.clone() }, diagnostics }
    }
}

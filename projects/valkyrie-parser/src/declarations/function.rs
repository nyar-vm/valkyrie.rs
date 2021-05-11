use super::*;
use crate::TypeReturnNode;

impl crate::DefineFunctionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<FunctionDeclaration> {
        let mut errors = vec![];
        let annotations = self.annotation_head.annotations(ctx).recover(&mut errors)?;
        let returning = self.function_body.build_return(ctx).recover(&mut errors)?;
        Success {
            value: FunctionDeclaration {
                name: self.namepath.build(ctx),
                kind: self.kw_function.build(),
                annotations,
                generic: None,
                arguments: Default::default(),
                r#return: returning,
                body: Default::default(),
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

impl crate::FunctionBodyNode {
    fn build_return(&self, ctx: &ProgramContext) -> Validation<FunctionReturnNode> {
        let mut errors = vec![];
        let typing = match &self.type_return {
            Some(s) => Some(s.type_expression.build(ctx)?),
            None => None,
        };
        Success { value: FunctionReturnNode { typing, effect: vec![] }, diagnostics: errors }
    }
}

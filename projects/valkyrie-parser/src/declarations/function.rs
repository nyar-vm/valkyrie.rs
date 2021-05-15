use super::*;
use crate::TypeReturnNode;

impl crate::DefineFunctionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<FunctionDeclaration> {
        let mut errors = vec![];
        let annotations = self.annotation_head.annotations(ctx).recover(&mut errors)?;
        let returning = self.function_middle.returns(ctx).recover(&mut errors)?;
        let body = self.continuation.build(ctx).recover(&mut errors)?;
        Success {
            value: FunctionDeclaration {
                name: self.namepath.build(ctx),
                kind: self.kw_function.build(),
                annotations,
                generic: None,
                arguments: Default::default(),
                returns: returning,
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
    pub fn returns(&self, ctx: &ProgramContext) -> Validation<FunctionReturnNode> {
        let mut errors = vec![];
        let typing = match &self.type_return {
            Some(s) => Some(s.type_expression.build(ctx)?),
            None => None,
        };
        Success { value: FunctionReturnNode { typing, effect: vec![] }, diagnostics: errors }
    }
}

impl crate::ContinuationNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<StatementBlock> {
        let mut diagnostics = vec![];
        let mut terms = vec![];
        for term in &self.main_statement {
            term.build(ctx).append(&mut terms, &mut diagnostics)
        }
        Success { value: StatementBlock { terms, span: self.span.clone() }, diagnostics }
    }
}

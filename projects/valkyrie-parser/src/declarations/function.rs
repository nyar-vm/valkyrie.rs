use super::*;

impl crate::DefineFunctionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<FunctionDeclaration> {
        let mut errors = vec![];
        // let terms = self.function_body.build(ctx).recover(&mut errors)?;
        let annotations = self.annotation_head.annotations(ctx).recover(&mut errors)?;
        Success {
            value: FunctionDeclaration {
                r#type: self.kw_function.build(),
                namepath: self.namepath.build(ctx),
                annotations,
                generic: None,
                arguments: Default::default(),
                r#return: None,
                body: Default::default(),
            },
            diagnostics: errors,
        }
    }
}

impl crate::KwFunctionNode {
    pub fn build(&self) -> FunctionType {
        match self {
            Self::Micro => FunctionType::Micro,
            Self::Macro => FunctionType::Macro,
        }
    }
}

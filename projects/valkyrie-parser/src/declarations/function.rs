use super::*;

impl crate::DefineFunctionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<FunctionDeclaration> {
        Success {
            value: FunctionDeclaration {
                r#type: self.kw_function.build(),
                namepath: self.namepath.build(ctx),
                modifiers: vec![],
                attributes: None,
                generic: None,
                arguments: Default::default(),
                r#return: None,
                body: Default::default(),
            },
            diagnostics: vec![],
        }
    }
}

impl KwFunctionNode {
    pub fn build(&self) -> FunctionType {
        match self {
            Self::Micro => FunctionType::Micro,
            Self::Macro => FunctionType::Macro,
        }
    }
}

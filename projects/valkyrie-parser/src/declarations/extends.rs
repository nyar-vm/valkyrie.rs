use super::*;

impl DefineExtendsNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExtendsStatement> {
        Success { value: ExtendsStatement { methods: vec![] }, diagnostics: vec![] }
    }
}

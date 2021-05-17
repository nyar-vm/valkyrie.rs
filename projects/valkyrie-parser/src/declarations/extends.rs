use super::*;

impl crate::DefineExtendsNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ExtendsStatement> {
        Success { value: ExtendsStatement { methods: vec![] }, diagnostics: vec![] }
    }
}

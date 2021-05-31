use super::*;

impl crate::DefineExtendsNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<ExtendsStatement> {
        Ok(ExtendsStatement { methods: vec![] })
    }
}

use super::*;

impl crate::DefineExtendsNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ExtendsStatement> {
        Ok(ExtendsStatement { body: vec![] })
    }
}

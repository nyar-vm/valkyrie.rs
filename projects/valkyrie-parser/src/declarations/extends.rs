use super::*;

impl crate::DefineExtendsNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ExtendsStatement> {
        Ok(ExtendsStatement { implements: build_type_hint(&self.type_hint, ctx), body: self.trait_block.build(ctx) })
    }
}

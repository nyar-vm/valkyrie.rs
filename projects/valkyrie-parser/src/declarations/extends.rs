use super::*;
use crate::TypeHintNode;

impl crate::DefineExtendsNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ExtendsStatement> {
        Ok(ExtendsStatement { body: self.class_block.build(ctx)? })
    }
}

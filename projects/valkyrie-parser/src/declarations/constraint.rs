use super::*;
use crate::DefineClassNode;

impl crate::DefineConstraintNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ConstraintDeclaration> {
        Ok(ConstraintDeclaration {})
    }
}

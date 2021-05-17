use super::*;

impl crate::ProceduralCallNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<BooleanNode> {
        let value = BooleanNode { value: false, span: Default::default() };
        Success { value, diagnostics: vec![] }
    }
}

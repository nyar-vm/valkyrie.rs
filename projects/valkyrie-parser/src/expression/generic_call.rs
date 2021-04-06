use super::*;
use crate::utils::build_arguments;

impl crate::GenericCallNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<GenericCallNode> {
        let monadic = self.op_and_then.is_some();
        let associated = match &self.namepath {
            Some(s) => s.build(ctx).names,
            None => {
                vec![]
            }
        };

        Success {
            value: GenericCallNode {
                monadic,
                base: Default::default(),
                terms: self.tuple_terms.build(ctx)?,
                associated,
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}

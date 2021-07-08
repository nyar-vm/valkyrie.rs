use super::*;

impl crate::IfGuardNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Option<ExpressionKind> {
        match &self.condition {
            Some(s) => match s.build(ctx) {
                Ok(o) => Some(o),
                Err(e) => {
                    ctx.add_error(e);
                    None
                }
            },
            None => None,
        }
    }
}

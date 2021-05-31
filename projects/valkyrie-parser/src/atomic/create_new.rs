use super::*;

impl crate::NewStatementNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<ConstructNewNode> {
        let base = self.namepath.build(ctx);
        let generics = match &self.generic_hide {
            Some(s) => vec![s.build(ctx)?],
            None => vec![],
        };
        let arguments = match &self.tuple_literal {
            Some(s) => s.build(ctx)?,
            None => TupleNode::default(),
        };
        // let returns = self.function_middle.returns(ctx).recover(&mut errors)?;
        Ok(ConstructNewNode {
            modifiers: vec![],
            namepath: self.namepath.build(ctx),
            generics,
            arguments,
            body: Default::default(),
            span: self.span.clone(),
        })
    }
}

use super::*;

impl crate::NewStatementNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ConstructNewNode> {
        let namepath = self.namepath.build(ctx);
        let generics = match &self.generic_hide {
            Some(s) => vec![s.build(ctx)?],
            None => vec![],
        };
        let arguments = match &self.tuple_literal {
            Some(s) => s.build(ctx)?.terms,
            None => ArgumentsList::default(),
        };
        // let returns = self.function_middle.returns(ctx).recover(&mut errors)?;
        Ok(ConstructNewNode {
            annotations: self.annotations(ctx),
            namepath,
            generics,
            arguments,
            body: Default::default(),
            span: self.span.clone(),
        })
    }
    fn annotations(&self, ctx: &mut ProgramState) -> AnnotationNode {
        let mut out = AnnotationNode::default();
        for term in &self.modifier_ahead {
            out.modifiers.terms.push(term.build(ctx))
        }
        out
    }
}

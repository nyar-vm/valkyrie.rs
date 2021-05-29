use super::*;

impl crate::GenericCallNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<GenericCallNode> {
        let monadic = self.op_and_then.is_some();
        let associated = match &self.namepath {
            Some(s) => s.build(ctx).names,
            None => {
                vec![]
            }
        };
        let term = GenericCallTerm::Generic(self.generic_terms.build(ctx)?);
        Success {
            value: GenericCallNode { monadic, base: Default::default(), term, span: self.span.clone() },
            diagnostics: vec![],
        }
    }
}
impl crate::GenericHideNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Validation<GenericCallTerm> {
        Success { value: GenericCallTerm::Generic(self.generic_terms.build(ctx)?), diagnostics: vec![] }
    }
    pub(crate) fn build_call(&self, ctx: &mut ProgramState) -> Validation<GenericCallNode> {
        Success {
            value: GenericCallNode {
                monadic: false,
                base: Default::default(),
                term: self.build(ctx)?,
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}

impl crate::GenericTermsNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ArgumentsList> {
        let mut errors = vec![];
        let mut terms = vec![];
        for x in &self.generic_pair {
            x.build(ctx).append(&mut terms, &mut errors)
        }
        Success { value: ArgumentsList { terms }, diagnostics: errors }
    }
}
impl crate::GenericPairNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<TupleTermNode> {
        let key = self.get_key(ctx);
        let value = self.type_expression.build(ctx)?;
        Success { value: TupleTermNode { key, value }, diagnostics: vec![] }
    }
    fn get_key(&self, ctx: &mut ProgramState) -> Option<IdentifierNode> {
        Some(self.identifier.as_ref()?.build(ctx))
    }
}

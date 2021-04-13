use super::*;

impl crate::DotCallNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<DotCallNode> {
        let monadic = self.op_and_then.is_some();
        Success {
            value: DotCallNode {
                monadic,
                base: Default::default(),
                term: self.dot_call_item.build(ctx)?,
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}

impl DotCallItemNode {
    pub fn build(&self, ctx: &ProgramContext) -> Result<DotCallTerm, NyarError> {
        match self {
            DotCallItemNode::Namepath(v) => Ok(DotCallTerm::Symbol(v.build(ctx))),
            DotCallItemNode::Integer(v) => {
                let u = usize::from_str(&v.text)?;
                Ok(DotCallTerm::index(u))
            }
        }
    }
}

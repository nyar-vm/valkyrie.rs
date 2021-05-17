use super::*;

impl crate::DotCallNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<DotCallNode> {
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

impl crate::DotCallItemNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<DotCallTerm, NyarError> {
        match self {
            Self::Namepath(v) => Ok(DotCallTerm::Symbol(v.build(ctx))),
            Self::Integer(v) => {
                let u = usize::from_str(&v.text)?;
                Ok(DotCallTerm::index(u))
            }
        }
    }
}

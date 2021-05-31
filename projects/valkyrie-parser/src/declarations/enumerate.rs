use super::*;

impl crate::DefineEnumerateNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<FlagDeclaration> {
        let mut terms = vec![];
        for term in &self.flag_term {
            match term.build(ctx) {
                Ok(Some(s)) => {
                    terms.push(s);
                }
                Ok(None) => {}
                Err(e) => {
                    ctx.add_error(e);
                }
            }
        }
        let annotations = self.annotation_head.annotations(ctx)?;
        Ok(FlagDeclaration {
            name: self.identifier.build(ctx),
            kind: self.kw_flags.build(),
            annotations,
            layout: None,
            implements: vec![],
            body: terms,
            span: self.span.clone(),
        })
    }
}

impl crate::KwFlagsNode {
    pub fn build(&self) -> FlagKind {
        match self {
            Self::Enum => FlagKind::Exclusive,
            Self::Flags => FlagKind::Juxtapose,
        }
    }
}
impl crate::FlagTermNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<Option<FlagTerm>> {
        let value = match self {
            Self::ProceduralCall(v) => v.build(ctx)?.into(),
            Self::DefineMethod(v) => v.build(ctx)?.into(),
            Self::FlagField(v) => v.build(ctx)?.into(),
            Self::EosFree(_) => return Ok(None),
        };
        Ok(Some(value))
    }
}

impl crate::FlagFieldNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<EncodeDeclaration> {
        let name = self.identifier.build(ctx);
        let value = match &self.main_expression {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        Ok(EncodeDeclaration { documentation: Default::default(), name, value, span: self.span.clone() })
    }
}

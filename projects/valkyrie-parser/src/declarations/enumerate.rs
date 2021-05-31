use super::*;

impl crate::DefineEnumerateNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<FlagDeclaration> {
        let mut terms = vec![];
        for term in &self.flag_term {
            match term {
                FlagTermNode::ProceduralCall(v) => terms.push(v.build(ctx)?.into()),
                FlagTermNode::DefineMethod(v) => terms.push(v.build(ctx)?.into()),
                FlagTermNode::FlagField(v) => terms.push(v.build(ctx)?.into()),
                FlagTermNode::EosFree(_) => {}
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

impl crate::FlagFieldNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<EncodeDeclaration> {
        let name = self.identifier.build(ctx);
        let value = match &self.main_expression {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        Success {
            value: EncodeDeclaration { documentation: Default::default(), name, value, span: self.span.clone() },
            diagnostics: vec![],
        }
    }
}

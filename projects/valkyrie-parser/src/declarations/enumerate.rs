use super::*;

impl crate::DefineEnumerateNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<FlagDeclaration> {
        let mut terms = vec![];
        for term in &self.flag_term {
            match term.build(ctx) {
                Ok(s) => terms.extend(s),
                Err(e) => ctx.add_error(e),
            }
        }
        Ok(FlagDeclaration {
            annotations: self.annotation_head.annotations(ctx),
            name: self.identifier.build(ctx),
            kind: self.kw_flags.build(),
            layout: None,
            implements: None,
            body: terms,
            span: self.span.clone(),
        })
    }
}

impl crate::KwFlagsNode {
    pub(crate) fn build(&self) -> FlagKind {
        match self {
            Self::Enum => FlagKind::Enumerate,
            Self::Flags => FlagKind::Flags,
        }
    }
}
impl crate::FlagTermNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<Option<FlagTerm>> {
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
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<EncodeDeclaration> {
        Ok(EncodeDeclaration {
            annotations: Default::default(),
            name: self.identifier.build(ctx),
            value: self.value(ctx),
            span: self.span.clone(),
        })
    }
    fn value(&self, ctx: &mut ProgramState) -> Option<ExpressionKind> {
        match self.main_expression.as_ref()?.build(ctx) {
            Ok(o) => Some(o),
            Err(e) => {
                ctx.add_error(e);
                None
            }
        }
    }
}

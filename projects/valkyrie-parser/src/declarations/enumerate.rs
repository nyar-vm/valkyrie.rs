use super::*;

impl crate::DefineEnumerateNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<FlagDeclaration> {
        let mut terms = vec![];
        let mut errors = vec![];
        for term in &self.flag_term {
            match term {
                FlagTermNode::ProceduralCall(_) => {}
                FlagTermNode::DefineMethod(v) => v.build(ctx).map(FlagTerm::Method).append(&mut terms, &mut errors),
                FlagTermNode::FlagField(v) => v.build(ctx).map(FlagTerm::Encode).append(&mut terms, &mut errors),
                FlagTermNode::EosFree(_) => {}
            }
        }
        let annotations = self.annotation_head.annotations(ctx).recover(&mut errors)?;
        Success {
            value: FlagDeclaration {
                name: self.identifier.build(ctx),
                kind: self.kw_flags.build(),
                annotations,
                layout: None,
                implements: vec![],
                body: Default::default(),
                span: self.span.clone(),
            },
            diagnostics: errors,
        }
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
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<EncodeDeclaration> {
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

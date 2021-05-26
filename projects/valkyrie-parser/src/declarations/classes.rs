use super::*;

impl crate::DefineClassNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ClassDeclaration> {
        let mut errors = vec![];
        let terms = self.class_block.build(ctx).recover(&mut errors)?;
        let annotations = self.annotation_head.annotations(ctx).recover(&mut errors)?;
        Success {
            value: ClassDeclaration {
                kind: self.kw_class.build(),
                annotations,
                name: self.identifier.build(ctx),
                generic: None,
                base_classes: None,
                auto_traits: vec![],
                terms,
                span: self.span.clone(),
            },
            diagnostics: errors,
        }
    }
}

impl crate::ClassBlockNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<Vec<ClassTerm>> {
        let mut terms = vec![];
        let mut errors = vec![];
        for term in &self.class_term {
            match term {
                crate::ClassTermNode::ProceduralCall(_) => {}
                crate::ClassTermNode::DefineDomain(v) => v.build(ctx).map(ClassTerm::Domain).append(&mut terms, &mut errors),
                crate::ClassTermNode::DefineField(v) => v.build(ctx).map(ClassTerm::Field).append(&mut terms, &mut errors),
                crate::ClassTermNode::DefineMethod(v) => v.build(ctx).map(ClassTerm::Method).append(&mut terms, &mut errors),
                crate::ClassTermNode::EosFree(_) => {}
            }
        }
        Success { value: terms, diagnostics: errors }
    }
}
impl crate::KwClassNode {
    pub fn build(&self) -> ClassKind {
        match self {
            Self::Class => ClassKind::Class,
            Self::Structure => ClassKind::Structure,
        }
    }
}
impl crate::DefineFieldNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<FieldDeclaration> {
        let mut errors = vec![];
        let name = self.identifier.build(ctx);
        let annotations = self.annotation_mix.annotations(ctx).recover(&mut errors)?;
        Success {
            value: FieldDeclaration { annotations, name, typing: None, default: None, span: self.span.clone() },
            diagnostics: errors,
        }
    }
}

impl crate::DefineMethodNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<MethodDeclaration> {
        let mut diagnostics = vec![];
        let name = self.namepath.build(ctx);
        let body = match &self.continuation {
            Some(s) => Some(s.build(ctx).recover(&mut diagnostics)?),
            None => None,
        };
        let parameters = self.function_middle.parameters(ctx).recover(&mut diagnostics)?;
        let generic = self.function_middle.generic(ctx).recover(&mut diagnostics)?;
        let returns = self.function_middle.returns(ctx).recover(&mut diagnostics)?;
        let annotations = self.annotation_mix.annotations(ctx).recover(&mut diagnostics)?;
        Success {
            value: MethodDeclaration { annotations, name, generic, parameters, returns, body, span: self.span.clone() },
            diagnostics,
        }
    }
}

impl crate::DefineDomainNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<DomainDeclaration> {
        // let name = self.class_block;
        Success {
            value: DomainDeclaration { body: self.class_block.build(ctx)?, span: self.span.clone() },
            diagnostics: vec![],
        }
    }
}

use super::*;

impl crate::DefineClassNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<ClassDeclaration> {
        let terms = self.class_block.build(ctx)?;
        let annotations = self.annotation_head.annotations(ctx)?;
        Ok(ClassDeclaration {
            kind: self.kw_class.build(),
            annotations,
            name: self.identifier.build(ctx),
            generic: None,
            base_classes: None,
            auto_traits: vec![],
            terms,
            span: self.span.clone(),
        })
    }
}

impl crate::ClassBlockNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<Vec<ClassTerm>> {
        let mut terms = vec![];
        for term in &self.class_term {
            match term.build(ctx) {
                Ok(Some(s)) => terms.push(s),
                Ok(None) => {}
                Err(e) => ctx.add_error(e),
            }
        }
        Ok(terms)
    }
}

impl crate::ClassTermNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<Option<ClassTerm>> {
        match self {
            Self::ProceduralCall(v) => Ok(Some(ClassTerm::Macro(v.build(ctx)?))),
            Self::DefineDomain(v) => Ok(Some(ClassTerm::Domain(v.build(ctx)?))),
            Self::DefineField(v) => Ok(Some(ClassTerm::Field(v.build(ctx)?))),
            Self::DefineMethod(v) => Ok(Some(ClassTerm::Method(v.build(ctx)?))),
            Self::EosFree(_) => Ok(None),
        }
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
    pub fn build(&self, ctx: &mut ProgramState) -> Result<FieldDeclaration> {
        let name = self.identifier.build(ctx);
        let annotations = self.annotation_mix.annotations(ctx)?;
        Ok(FieldDeclaration { annotations, name, typing: None, default: None, span: self.span.clone() })
    }
}

impl crate::DefineMethodNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<MethodDeclaration> {
        let name = self.namepath.build(ctx);
        let body = match &self.continuation {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        let parameters = self.function_middle.parameters(ctx)?;
        let generic = self.function_middle.generics(ctx)?;
        let returns = self.function_middle.returns(ctx)?;
        let annotations = self.annotation_mix.annotations(ctx)?;
        Ok(MethodDeclaration { annotations, name, generics: generic, parameters, returns, body, span: self.span.clone() })
    }
}

impl crate::DefineDomainNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<DomainDeclaration> {
        // let name = self.class_block;
        Ok(DomainDeclaration { body: self.class_block.build(ctx)?, span: self.span.clone() })
    }
}

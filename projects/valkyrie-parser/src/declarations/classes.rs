use super::*;
use crate::ClassTermNode;

impl crate::DefineClassNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ClassDeclaration> {
        let mut terms = vec![];
        let mut errors = vec![];
        for term in &self.class_block.class_term {
            match term {
                ClassTermNode::DefineDomain(_) => {}
                ClassTermNode::DefineField(_) => {}
                ClassTermNode::DefineMethod(v) => v.build(ctx).map(ClassTerm::Method).append(&mut terms, &mut errors),
                ClassTermNode::EosFree(_) => {}
                ClassTermNode::ProceduralCall(_) => {}
            }
        }
        Success {
            value: ClassDeclaration {
                kind: self.kw_class.build(),
                modifiers: self.annotation_head.get_modifiers(ctx),
                name: self.identifier.build(ctx),
                generic: None,
                base_classes: None,
                auto_traits: vec![],
                body: terms,
                span: self.span.clone(),
            },
            diagnostics: errors,
        }
    }
}

impl KwClassNode {
    pub fn build(&self) -> ClassKind {
        match self {
            Self::Class => ClassKind::Class,
            Self::Structure => ClassKind::Structure,
        }
    }
}
impl crate::DefineMethodNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<MethodDeclaration> {
        let name = self.namepath.build(ctx);
        Success {
            value: MethodDeclaration {
                document: Default::default(),
                modifiers: Default::default(),
                method_name: name,
                generic: None,
                arguments: Default::default(),
                return_type: None,
                effect_type: None,
                body: None,
            },
            diagnostics: vec![],
        }
    }
}

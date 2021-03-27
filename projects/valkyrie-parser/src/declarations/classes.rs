use crate::{KwClassNode, ProgramContext};
use nyar_error::{Success, Validation};
use valkyrie_ast::{ClassDeclaration, ClassKind, IdentifierNode};

impl crate::DefineClassNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ClassDeclaration> {
        Success {
            value: ClassDeclaration {
                kind: self.kw_class.build(),
                modifiers: Default::default(),
                name: self.identifier.build(ctx),
                generic: None,
                base_classes: None,
                auto_traits: vec![],
                fields: vec![],
                methods: vec![],
                span: self.span.clone(),
            },
            diagnostics: vec![],
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

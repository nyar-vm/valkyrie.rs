use crate::{KwClassNode, KwFlagsNode, ProgramContext};
use nyar_error::{Success, Validation};
use valkyrie_ast::{ClassDeclaration, ClassKind, FlagsDeclaration, FlagsKind, IdentifierNode, NamePathNode};

impl crate::DefineEnumerateNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<FlagsDeclaration> {
        Success {
            value: FlagsDeclaration {
                documentation: Default::default(),
                kind: FlagsKind::Enumerate,
                namepath: NamePathNode { names: vec![] },
                modifiers: vec![],
                layout: None,
                implements: vec![],
                body: Default::default(),
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}

impl KwFlagsNode {
    pub fn build(&self) -> ClassKind {
        match self {
            Self::Enum => ClassKind::Class,
            Self::Flags => ClassKind::Structure,
        }
    }
}

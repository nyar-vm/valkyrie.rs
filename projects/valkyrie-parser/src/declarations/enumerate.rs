use super::*;

impl crate::DefineEnumerateNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<FlagDeclaration> {
        Success {
            value: FlagDeclaration {
                documentation: Default::default(),
                kind: self.kw_flags.build(),
                name: self.identifier.build(ctx),
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
    pub fn build(&self) -> FlagsKind {
        match self {
            Self::Enum => FlagsKind::Exclusive,
            Self::Flags => FlagsKind::Juxtapose,
        }
    }
}

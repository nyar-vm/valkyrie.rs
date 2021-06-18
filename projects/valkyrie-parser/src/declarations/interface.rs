use super::*;

impl crate::DefineTraitNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<TraitDeclaration> {
        Ok(TraitDeclaration {
            kind: self.kw_trait.build(),
            name: self.identifier.build(ctx),
            generic: self.define_generic.as_ref().map(|s| s.build(ctx)),
            terms: vec![],
        })
    }
}

impl crate::KwTraitNode {
    pub(crate) fn build(&self) -> TraitKind {
        TraitKind::Trait
    }
}

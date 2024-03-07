use crate::{helpers::Hir2Mir, ResolveContext, ValkyrieField};
use nyar_wasm::Identifier;
use valkyrie_ast::{ClassTerm, FieldDeclaration, UnionDeclaration, UnionTerm, VariantDeclaration};

impl Hir2Mir for UnionDeclaration {
    type Output = ValkyrieUnion;
    fn to_mir(self, ctx: &mut ResolveContext) -> nyar_error::Result<Self::Output> {
        for variant in self.body {
            match variant {
                UnionTerm::Macro(_) => {
                    todo!()
                }
                UnionTerm::Variant(v) => {
                    let a = v.to_mir(ctx)?;
                }
                UnionTerm::Method(_) => {
                    todo!()
                }
            }
        }
        Ok(crate::variants::ValkyrieUnion { symbol: Default::default() })
    }
}

impl Hir2Mir for VariantDeclaration {
    type Output = ValkyrieUnionItem;
    fn to_mir(self, ctx: &mut ResolveContext) -> nyar_error::Result<Self::Output> {
        for variant in self.body {
            match variant {
                ClassTerm::Macro(_) => {
                    todo!()
                }
                ClassTerm::Field(v) => {
                    let field = v.to_mir(ctx)?;
                }
                ClassTerm::Method(_) => {
                    todo!()
                }
                ClassTerm::Domain(_) => {
                    todo!()
                }
            }
        }
        Ok(crate::variants::ValkyrieUnionItem {})
    }
}

#[derive(Clone, Debug)]
pub struct ValkyrieUnion {
    pub symbol: Identifier,
}

pub struct ValkyrieUnionItem {}

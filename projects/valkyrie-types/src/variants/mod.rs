use crate::{helpers::Hir2Mir, ResolveContext, ValkyrieField};
use indexmap::IndexMap;
use nyar_wasm::Identifier;
use std::sync::Arc;
use valkyrie_ast::{ClassTerm, FieldDeclaration, UnionDeclaration, UnionTerm, VariantDeclaration};

mod codegen;
mod parser;

impl Hir2Mir for UnionDeclaration {
    type Output = ValkyrieUnion;
    fn to_mir(self, ctx: &mut ResolveContext) -> nyar_error::Result<Self::Output> {
        let name = ctx.get_name_path(&self.name);
        let mut output = crate::variants::ValkyrieUnion { union_name: name, variants: Default::default() };
        for item in self.body {
            match item {
                UnionTerm::Macro(_) => {
                    todo!()
                }
                UnionTerm::Variant(v) => {
                    let variant = v.to_mir(ctx)?;
                    match output.variants.insert(variant.variant_name.clone(), variant) {
                        Some(_) => {
                            panic!("dup variant")
                        }
                        None => {}
                    }
                }
                UnionTerm::Method(_) => {
                    todo!()
                }
            }
        }
        Ok(output)
    }
}

impl Hir2Mir for VariantDeclaration {
    type Output = ValkyrieUnionItem;
    fn to_mir(self, ctx: &mut ResolveContext) -> nyar_error::Result<Self::Output> {
        let name: Arc<str> = Arc::from(self.name.name.as_str());
        let mut output = ValkyrieUnionItem { variant_name: name.clone(), wasi_alias: name.clone(), fields: Default::default() };
        for item in self.body {
            match item {
                ClassTerm::Macro(_) => {
                    todo!()
                }
                ClassTerm::Field(v) => {
                    let field = v.to_mir(ctx)?;
                    output.fields.insert(field.field_name.clone(), field);
                }
                ClassTerm::Method(_) => {
                    todo!()
                }
                ClassTerm::Domain(_) => {
                    todo!()
                }
            }
        }
        Ok(output)
    }
}

#[derive(Clone, Debug)]
pub struct ValkyrieUnion {
    pub union_name: Identifier,
    pub variants: IndexMap<Arc<str>, ValkyrieUnionItem>,
}

#[derive(Clone, Debug)]
pub struct ValkyrieUnionItem {
    pub variant_name: Arc<str>,
    pub wasi_alias: Arc<str>,
    pub fields: IndexMap<Arc<str>, ValkyrieField>,
}

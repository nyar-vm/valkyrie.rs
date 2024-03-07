use super::*;
use nyar_wasm::WasiModule;
use std::str::FromStr;
use valkyrie_ast::{ArgumentTerm, AttributeTerm, MethodDeclaration};

impl Hir2Mir for ClassDeclaration {
    type Output = ();
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        let symbol = ctx.get_name_path(&self.name);
        let mut class = ValkyrieClass::new(symbol);

        match ctx.get_foreign_module(&self.annotations, "class", "resource") {
            Some((module, name)) => {
                class.category = ValkyrieClassCategory::Resource { wasi_module: module, wasi_name: name };
            }
            None => {}
        }

        for x in self.terms {
            match x {
                ClassTerm::Macro(_) => {
                    todo!()
                }
                ClassTerm::Field(v) => {
                    let field = v.to_mir(ctx)?;
                    match class.fields.insert(field.field_name.clone(), field) {
                        Some(s) => {
                            unimplemented!()
                        }
                        None => {}
                    }
                }
                ClassTerm::Method(v) => {
                    let method = v.to_mir(ctx)?;
                    match class.methods.insert(method.method_name.clone(), method) {
                        Some(s) => {
                            unimplemented!()
                        }
                        None => {}
                    }
                }
                ClassTerm::Domain(_) => {
                    todo!()
                }
            }
        }
        *ctx += class;

        Ok(())
    }
}

impl AddAssign<ValkyrieClass> for ResolveContext {
    fn add_assign(&mut self, rhs: ValkyrieClass) {
        self.items.insert(rhs.symbol.clone(), ModuleItem::Structure(rhs));
    }
}

impl Hir2Mir for FieldDeclaration {
    type Output = ValkyrieField;
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        let (field_name, wasi_alias) = ctx.get_field_alias(&self.name, &self.annotations)?;

        Ok(ValkyrieField { field_name, wasi_alias })
    }
}
impl Hir2Mir for MethodDeclaration {
    type Output = ValkyrieMethod;
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        let (method_name, wasi_alias) = ctx.get_field_alias(&self.name, &self.annotations)?;
        Ok(ValkyrieMethod { method_name, wasi_alias })
    }
}

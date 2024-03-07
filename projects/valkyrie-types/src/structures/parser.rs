use super::*;
use nyar_wasm::WasiModule;
use std::str::FromStr;
use valkyrie_ast::{ArgumentTerm, AttributeTerm, MethodDeclaration};

impl Hir2Mir for ClassDeclaration {
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        let symbol = ctx.get_name_path(&self.name);
        let mut class = ValkyrieStructure::new(symbol);

        match self.annotations.attributes.get("ffi") {
            Some(s) => {
                if !self.annotations.modifiers.contains("resource") {
                    panic!("must resource class")
                }
                match s.arguments.terms.as_slice() {
                    [module, name] => {
                        let module = module.value.as_text().unwrap();
                        let name = name.value.as_text().unwrap();
                        class.external_resource = Some(WasiResource {
                            symbol: class.symbol.clone(),
                            wasi_module: WasiModule::from_str(module).unwrap(),
                            wasi_name: name.to_string(),
                        })
                    }
                    _ => panic!("invalid ffi attribute"),
                }
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

                    // let mut field = ValkyrieField::new(&f.name);
                    // field.typing = f.typing;
                    //
                    // class.add_field(field).ok();
                }
                ClassTerm::Method(v) => {
                    let field = v.to_mir(ctx)?;
                }
                ClassTerm::Domain(_) => {
                    todo!()
                }
            }
        }

        match ctx.items.entry(class.symbol.clone()) {
            Entry::Occupied(e) => {
                todo!()
            }
            Entry::Vacant(e) => {
                e.insert(ModuleItem::Structure(class));
            }
        }
        Ok(())
    }
}
impl Hir2Mir for FieldDeclaration {
    type Output = ValkyrieField;
    fn to_mir(self, ctx: &mut ResolveContext) -> nyar_error::Result<Self::Output> {
        Ok(ValkyrieField { name: Arc::from("?"), wasi_name: Arc::from("?") })
    }
}
impl Hir2Mir for MethodDeclaration {
    type Output = ValkyrieMethod;
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        Ok(ValkyrieMethod {})
    }
}

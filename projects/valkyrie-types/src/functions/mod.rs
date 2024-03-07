use crate::{helpers::Hir2Mir, ModuleItem, ResolveContext};
use nyar_wasm::{Identifier, WasiModule};
use std::sync::Arc;
use valkyrie_ast::FunctionDeclaration;

mod codegen;
mod parser;

#[derive(Debug)]
pub struct ValkyrieExternalFunction {
    pub external_name: Identifier,
    pub wasi_module: WasiModule,
    pub wasi_name: Arc<str>,
}

pub struct ValkyrieFunction {
    pub function_name: Identifier,
}

impl Hir2Mir for FunctionDeclaration {
    type Output = ModuleItem;
    fn to_mir(self, ctx: &mut ResolveContext) -> nyar_error::Result<Self::Output> {
        match ctx.get_foreign_module(&self.annotations, "function", "external") {
            Some((wasi_module, wasi_name)) => {
                return Ok(ModuleItem::External(ValkyrieExternalFunction {
                    external_name: Default::default(),
                    wasi_module,
                    wasi_name,
                }));
            }
            None => {}
        }
        Ok(ModuleItem::External(todo!()))
    }
}

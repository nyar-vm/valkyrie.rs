use crate::{helpers::Hir2Mir, ModuleItem, ResolveContext};
use nyar_wasm::{Identifier, WasiModule};
use std::sync::Arc;
use valkyrie_ast::FunctionDeclaration;

mod codegen;
mod parser;

#[derive(Debug)]
pub struct ValkyrieFunction {
    pub function_name: Identifier,
    pub category: ValkyrieFunctionCategory,
}

#[derive(Clone, Default, Debug)]
pub enum ValkyrieFunctionCategory {
    #[default]
    Normal,
    /// Represent an external wasi function
    External {
        /// The wasi module name
        wasi_module: WasiModule,
        /// The external function name
        wasi_name: Arc<str>,
    },
}

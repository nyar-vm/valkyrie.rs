use crate::{helpers::Hir2Mir, ModuleItem, ResolveContext};
use nyar_wasm::{Identifier, WasiModule};
use std::{ops::AddAssign, sync::Arc};
use valkyrie_ast::FunctionDeclaration;
mod arithmetic;
mod codegen;
mod parser;

/// The [function](), [`external` function](), [`extension` function](), [`overload` function] in valkyrie language
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

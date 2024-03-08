use crate::{
    helpers::{Hir2Mir, Mir2Lir},
    ModuleItem, ResolveContext,
};
use nyar_wasm::{DependentGraph, Identifier, WasiExport, WasiFunction, WasiImport, WasiModule};
use std::{ops::AddAssign, sync::Arc};
use valkyrie_ast::FunctionDeclaration;
mod arithmetic;
mod stage1_mir;
mod stage2_lir;

/// The [function](), [`external` function](), [`extension` function](), [`overload` function] in valkyrie language
#[derive(Debug)]
pub struct ValkyrieFunction {
    pub function_name: Identifier,
    /// The WASI import symbol if exists
    pub wasi_import: Option<WasiImport>,
    /// The WASI export symbol if exists
    pub wasi_export: Option<WasiExport>,
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

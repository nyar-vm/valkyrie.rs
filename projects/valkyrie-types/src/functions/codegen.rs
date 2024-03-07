use super::*;
use crate::helpers::Mir2Lir;
use nyar_wasm::{DependentGraph, WasiFunction};

impl Mir2Lir for ValkyrieFunction {
    fn to_lir(&self, ctx: &ResolveContext, graph: &mut DependentGraph) -> nyar_error::Result<Self::Output> {
        let fun = match &self.category {
            ValkyrieFunctionCategory::Normal => {
                todo!()
            }
            ValkyrieFunctionCategory::External { wasi_module, wasi_name } => {
                WasiFunction::external(wasi_module, wasi_name, &self.function_name)
            }
        };
        Ok(*graph += fun)
    }
}

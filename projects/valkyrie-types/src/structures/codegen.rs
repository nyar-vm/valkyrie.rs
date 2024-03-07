use super::*;
use crate::helpers::Mir2Lir;
use nyar_wasm::DependentGraph;

impl Mir2Lir for ValkyrieClass {
    fn to_lir(&self, ctx: &ResolveContext, graph: &mut DependentGraph) -> Result<Self::Output> {
        match &self.category {
            ValkyrieClassCategory::Structure => {}
            ValkyrieClassCategory::Resource { wasi_module, wasi_name } => {
                *graph += WasiResource {
                    symbol: self.symbol.clone(),
                    wasi_module: wasi_module.clone(),
                    wasi_name: wasi_name.clone(),
                };
            }
        }

        Ok(())
    }
}

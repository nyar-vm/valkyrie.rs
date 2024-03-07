use super::*;
use crate::helpers::Mir2Lir;
use nyar_wasm::DependentGraph;

impl Mir2Lir for ValkyrieResource {
    fn to_lir(&self, ctx: &ResolveContext, graph: &mut DependentGraph) -> Result<Self::Output> {
        *graph += WasiResource {
            symbol: self.symbol.clone(),
            wasi_module: self.wasi_module.clone(),
            wasi_name: self.wasi_name.clone(),
        };
        Ok(())
    }
}

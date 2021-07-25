use crate::{ModuleItem, ModuleResolver, ValkyrieStructure};
use nyar_wasm::{StructureType, WasmBuilder, WasmSymbol, WasmType};

impl ModuleResolver {
    pub fn build_wasm(&self) -> WasmBuilder {
        let mut builder = WasmBuilder::default();

        for item in self.items.values() {
            match item {
                ModuleItem::Imported(_) => {}
                ModuleItem::Structure(v) => builder.insert_type(WasmType::Structure(StructureType {
                    symbol: WasmSymbol::from(v.name()),
                    fields: Default::default(),
                    span: Default::default(),
                })),
                ModuleItem::External(_) => {}
            }
        }
        builder
    }
}

impl ValkyrieStructure {}

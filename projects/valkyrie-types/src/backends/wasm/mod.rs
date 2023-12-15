use crate::{FieldDefinition, ModuleItem, ModuleResolver, ValkyrieStructure};
use nyar_wasm::{FieldType, StructureType, WasmBuilder, WasmItem, WasmSymbol, WasmType};
use valkyrie_ast::ExpressionKind;

impl ModuleResolver {
    pub fn build_wasm(&self) -> WasmBuilder {
        let mut builder = WasmBuilder::default();

        for item in self.items.values() {
            match item {
                ModuleItem::Imported(_) => {}
                ModuleItem::Structure(v) => builder.register(v.as_wasm()),
                ModuleItem::External(_) => {}
            }
        }
        builder
    }
}

pub(crate) trait IntoWasm<Item> {
    fn as_wasm(&self) -> Item;
}

impl IntoWasm<StructureType> for ValkyrieStructure {
    fn as_wasm(&self) -> StructureType {
        let mut item = StructureType::new(self.name());
        for field in self.fields.values() {
            item.register_field(field.as_wasm())
        }
        item
    }
}
impl IntoWasm<FieldType> for FieldDefinition {
    fn as_wasm(&self) -> FieldType {
        match &self.typing {
            Some(s) => {
                let name = self.symbol.to_string();
                println!("name: {name}");

                FieldType::new(name).with_type(s.as_wasm())
            }
            None => {
                panic!("Run type infer first")
            }
        }
    }
}

impl IntoWasm<WasmType> for ExpressionKind {
    fn as_wasm(&self) -> WasmType {
        match self {
            ExpressionKind::Symbol(s) => WasmType::Reference { name: WasmSymbol::from(s.to_string()), nullable: false },
            _ => unimplemented!("Unknown type item {self:?}"),
        }
    }
}

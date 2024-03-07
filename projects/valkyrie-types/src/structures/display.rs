use super::*;

impl Debug for ValkyrieClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.category {
            ValkyrieClassCategory::Structure => {
                let debug = &mut f.debug_struct("Class");
                debug.field("symbol", &WrapDisplay::new(&self.symbol)).field("fields", &self.fields.values());
                debug.finish()
            }
            ValkyrieClassCategory::Resource { wasi_module, wasi_name } => {
                let debug = &mut f.debug_struct("Resource");
                debug.field("symbol", &self.symbol).field("module", &wasi_module).field("name", &wasi_name);
                debug.finish()
            }
        }
    }
}
impl Debug for ValkyrieField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field").field("name", &self.field_name).field("wasi", &self.wasi_alias).finish()
    }
}

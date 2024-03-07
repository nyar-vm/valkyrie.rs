use super::*;

impl Debug for ValkyrieClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self.category {
            ValkyrieClassCategory::Structure => "Class",
            ValkyrieClassCategory::Resource { .. } => "Resource",
        };
        let debug = &mut f.debug_struct(name);
        debug.field("symbol", &WrapDisplay::new(&self.symbol)).field("fields", &self.fields.values());
        match self.category {
            ValkyrieClassCategory::Structure => {}
            ValkyrieClassCategory::Resource { .. } => {}
        }
        debug.finish()
    }
}
impl Debug for ValkyrieField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field").field("name", &self.field_name).field("wasi", &self.wasi_alias).finish()
    }
}

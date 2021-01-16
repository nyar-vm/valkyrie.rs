use crate::types::ValkyrieMetaType;
use std::sync::Arc;

pub fn primitive_type(name: &str) -> Gc<ValkyrieMetaType> {
    let mut this = ValkyrieMetaType::default();
    this.set_namepath(name);
    Arc::new(this)
}

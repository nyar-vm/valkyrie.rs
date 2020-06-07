use crate::{types::ValkyrieMetaType, ValkyrieType, ValkyrieValue};
use std::sync::Arc;

pub struct ValkyrieImage {
    bytes: Vec<u8>,
    width: u32,
    height: u32,
    channel: u32,
}

impl ValkyrieType for ValkyrieImage {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::new("std.image.Image");
        // this.set_namepath("std.collection.SortedMap");
        // this.mut_generic_types().push(K::static_type());
        // this.mut_generic_types().push(V::static_type());
        Arc::new(this)
    }
}

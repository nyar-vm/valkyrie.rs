use std::sync::Arc;

use crate::{types::ValkyrieMetaType, ValkyrieClass, ValkyrieType, ValkyrieValue};

impl<T> ValkyrieType for Vec<T>
where
    T: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        let mut out = ValkyrieClass::list();
        for item in self {
            out.extend_one(item.boxed());
        }
        ValkyrieValue::Class(Arc::new(out))
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.collection.Array");
        this.mut_generic_types().push(T::static_type());
        Arc::new(this)
    }
}

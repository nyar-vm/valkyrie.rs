use std::{ops::Deref, sync::Arc};

use crate::{types::ValkyrieMetaType, ValkyrieType, ValkyrieValue};

impl<T> ValkyrieType for Arc<T>
where
    T: ValkyrieType,
{
    #[track_caller]
    fn boxed(self) -> ValkyrieValue {
        panic!("Arc<T> can't be not boxed")
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        self.as_ref().dynamic_type()
    }
}

use shredder::Gc;

use crate::{types::ValkyrieMetaType, ValkyrieType, ValkyrieValue, ValkyrieVariantType};

impl<T> ValkyrieType for Option<T>
where
    T: ValkyrieType + 'static,
{
    fn boxed(self) -> ValkyrieValue {
        let this = ValkyrieVariantType::new("std.primitive.Option");
        this.boxed()
    }
    fn dynamic_type(&self) -> Gc<ValkyrieMetaType>
    where
        Self: Sized,
    {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Option");
        this.mut_generic_types().push(T::static_type());
        Gc::new(this)
    }
}

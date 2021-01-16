use super::*;
use image::{DynamicImage, RgbaImage};

#[derive(Clone, Debug)]
pub struct ValkyrieImage {
    rgba: RgbaImage,
}

unsafe impl GcSafe for ValkyrieImage {}

unsafe impl Scan for ValkyrieImage {
    fn scan(&self, _: &mut Scanner<'_>) {}
}

impl ValkyrieImage {
    pub fn width(&self) -> ValkyrieValue {
        ValkyrieValue::from(self.rgba.width())
    }
    pub fn height(&self) -> ValkyrieValue {
        ValkyrieValue::from(self.rgba.height())
    }
    pub fn get_pixel(&self) {
        todo!()
    }
}

impl ValkyrieType for ValkyrieImage {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        // TODO: pixel generic
        let this = ValkyrieMetaType::new("std.image.Image");
        // this.set_namepath("std.collection.SortedMap");
        // this.mut_generic_types().push(K::static_type());
        // this.mut_generic_types().push(V::static_type());
        Gc::new(this)
    }
}

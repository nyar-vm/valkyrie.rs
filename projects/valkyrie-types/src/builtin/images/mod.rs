use crate::{types::ValkyrieMetaType, ValkyrieType, ValkyrieValue};
use image::DynamicImage;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ValkyrieImage {
    any_image: DynamicImage,
}

impl ValkyrieImage {
    pub fn width(&self) -> ValkyrieValue {
        ValkyrieValue::from(self.any_image.width())
    }
    pub fn height(&self) -> ValkyrieValue {
        ValkyrieValue::from(self.any_image.height())
    }
    pub fn get_pixel(&self) {
        match &self.any_image {
            DynamicImage::ImageLuma8(_) => {
                todo!()
            }
            DynamicImage::ImageLumaA8(_) => {
                todo!()
            }
            DynamicImage::ImageRgb8(_) => {
                todo!()
            }
            DynamicImage::ImageRgba8(_) => {
                todo!()
            }
            DynamicImage::ImageLuma16(_) => {
                todo!()
            }
            DynamicImage::ImageLumaA16(_) => {
                todo!()
            }
            DynamicImage::ImageRgb16(_) => {
                todo!()
            }
            DynamicImage::ImageRgba16(_) => {
                todo!()
            }
            DynamicImage::ImageRgb32F(_) => {
                todo!()
            }
            DynamicImage::ImageRgba32F(_) => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
}

impl ValkyrieType for ValkyrieImage {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        // TODO: pixel generic
        let this = ValkyrieMetaType::new("std.image.Image");
        // this.set_namepath("std.collection.SortedMap");
        // this.mut_generic_types().push(K::static_type());
        // this.mut_generic_types().push(V::static_type());
        Arc::new(this)
    }
}

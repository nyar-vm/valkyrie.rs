use crate::ValkyrieCodegen;

pub trait FromFrontend<Item> {
    fn build(&self, state: &mut ValkyrieCodegen) -> nyar_error::Result<Item>;
}

pub trait IntoBackend<Item> {
    fn build(&self, state: &mut ValkyrieCodegen) -> nyar_error::Result<Item>;
}

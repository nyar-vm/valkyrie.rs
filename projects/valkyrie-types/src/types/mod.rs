use crate::ValkyrieFunction;
use nyar_wasm::Identifier;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub enum ValkyrieType {
    Boolean,
    Integer,
    Function(Arc<Mutex<ValkyrieFunction>>),
    Unsourced(Identifier),
}

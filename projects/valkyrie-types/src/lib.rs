#![feature(associated_type_defaults)]
#![feature(extend_one)]

mod functions;
mod helpers;
mod modules;
mod structures;
mod variants;

pub use crate::{
    functions::ValkyrieExternalFunction,
    modules::{ModuleItem, ResolveContext, ValkyrieModule},
    structures::{ValkyrieClass, ValkyrieField, ValkyrieMethod},
    variants::{ValkyrieUnion, ValkyrieUnionItem},
};

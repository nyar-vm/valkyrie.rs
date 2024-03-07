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
    structures::{ValkyrieField, ValkyrieMethod, ValkyrieResource, ValkyrieStructure},
    variants::{ValkyrieUnion, ValkyrieUnionItem},
};
use std::{io::Write, str::FromStr};

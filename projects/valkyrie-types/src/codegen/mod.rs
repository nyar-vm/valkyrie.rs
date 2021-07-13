use crate::{Failure, FileCache, FileID, Success};
use nyar_error::{third_party::Url, NyarError, Result};
use nyar_wasm::ModuleBuilder;
use valkyrie_ast::{IdentifierNode, NamePathNode};

pub mod backend_wasm;

#[derive(Default)]
pub struct ValkyrieCodegen {
    module: ModuleBuilder,
    cache: FileCache,
    current_file: FileID,
    pub(crate) current_namespace: NamePathNode,
    errors: Vec<NyarError>,
}

impl ValkyrieCodegen {
    pub fn print_error(&self, error: NyarError) {
        match error.as_report().eprint(&self.cache) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

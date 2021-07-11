use crate::{Failure, FileCache, FileID, Success};
use nyar_error::{third_party::Url, NyarError, Result};
use nyar_wasm::ModuleBuilder;

pub mod backend_wasm;

#[derive(Default)]
pub struct ValkyrieWasmCodegen {
    module: ModuleBuilder,
    files: FileCache,
    current_file: FileID,
    errors: Vec<NyarError>,
}

impl ValkyrieWasmCodegen {
    pub fn print_error(&self, error: NyarError) {
        match error.as_report().eprint(&self.files) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

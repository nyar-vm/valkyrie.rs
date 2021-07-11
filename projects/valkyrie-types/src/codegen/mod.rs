use crate::{Failure, FileCache, Success};
use nyar_error::{third_party::Url, NyarError, Result};
use nyar_wasm::ModuleBuilder;
use valkyrie_ast::{ProgramRoot, StatementKind};
use valkyrie_parser::ProgramContext;

pub mod backend_wasm;

#[derive(Default)]
pub struct ValkyrieWasmCodegen {
    module: ModuleBuilder,
    files: FileCache,
    errors: Vec<NyarError>,
}

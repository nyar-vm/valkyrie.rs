use crate::{helpers::FromFrontend, types::field_type::FieldDefinition, ClassDefinition, FileCache, FileID};
use nyar_error::{NyarError, Result};
use nyar_wasm::{FieldType, ModuleBuilder, NyarType, StructureType, Symbol};
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    io::Write,
    mem::take,
    path::{Path, PathBuf},
    sync::Arc,
};
use valkyrie_ast::{ExpressionKind, FieldDeclaration, GenericCallTerm, NamePathNode, ProgramRoot, StatementKind};
use valkyrie_parser::ProgramContext;
pub mod backend_wasm;

#[derive(Default)]
pub struct ValkyrieCodegen {
    module: ModuleBuilder,
    cache: FileCache,
    current_file: FileID,
    pub(crate) interactive: bool,
    pub(crate) current_namespace: NamePathNode,
    imports: HashMap<Vec<Arc<str>>, Vec<Arc<str>>>,
    errors: Vec<NyarError>,
}

impl ValkyrieCodegen {
    pub fn print_error(&self, error: NyarError) {
        match error.as_report().eprint(&self.cache) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
    pub fn add_error<E: Into<NyarError>>(&mut self, error: E) {
        self.errors.push(error.into())
    }
}

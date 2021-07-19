use salsa::{
    cycle::CycleRecoveryStrategy, database::AsSalsaDatabase, key::DependencyIndex, runtime::local_state::QueryOrigin,
    storage::HasJarsDyn, Database, DatabaseKeyIndex, DbWithJar, DebugWithDb, Id, IngredientIndex, ParallelDatabase, Revision,
    Runtime,
};
use std::{
    fmt::Formatter,
    sync::{Arc, Mutex},
};

pub trait ValkyrieData: DbWithJar<ValkyrieJar> {}

#[salsa::jar(db = ValkyrieData)]
pub struct ValkyrieJar(
    // crate::compile::compile,
    // crate::ir::SourceProgram,
    // crate::ir::Program,
    // crate::ir::VariableId,
    // crate::ir::FunctionId,
    // crate::ir::Function,
    // crate::ir::Diagnostics,
    // crate::ir::Span,
    // crate::parser::parse_statements,
    // crate::type_check::type_check_program,
    // crate::type_check::type_check_function,
    // crate::type_check::find_function,
);

impl<Data> ValkyrieData for Data where Data: ?Sized + DbWithJar<ValkyrieJar> {}

#[derive(Default)]
#[salsa::db(ValkyrieJar)]
pub(crate) struct ValkyrieDatabase {
    storage: salsa::Storage<Self>,

    // The logs are only used for testing and demonstrating reuse:
    logs: Option<Arc<Mutex<Vec<String>>>>,
}

impl Database for ValkyrieDatabase {
    fn salsa_event(&self, event: salsa::Event) {
        // Log interesting events, if logging is enabled
        if let Some(logs) = &self.logs {
            // don't log boring events
            if let salsa::EventKind::WillExecute { .. } = event.kind {
                logs.lock().unwrap().push(format!("Event: {:?}", event.debug(self)));
            }
        }
    }
}
impl ParallelDatabase for ValkyrieDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(ValkyrieDatabase { storage: self.storage.snapshot(), logs: self.logs.clone() })
    }
}

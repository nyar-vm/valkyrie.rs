use crate::{
    helpers::{Hir2Mir, Mir2Lir},
    structures::ValkyrieStructure,
    ValkyrieExternalFunction, ValkyrieResource, ValkyrieUnion, ValkyrieUnionItem,
};
use im::HashMap;
use indexmap::IndexMap;
use nyar_error::{Failure, FileCache, FileID, NyarError, Result, Success};
use nyar_wasm::{CanonicalWasi, DependentGraph, Identifier, WasiResource};
use std::{
    borrow::Cow,
    fmt::{Debug, Formatter},
    mem::take,
    sync::Arc,
};
use valkyrie_ast::{
    FunctionDeclaration, IdentifierNode, NamePathNode, NamespaceDeclaration, ProgramRoot, StatementKind, UnionDeclaration,
};
use valkyrie_parser::{ProgramContext, StatementNode};

mod codegen;
mod display;
mod parser;

pub struct ValkyrieModule {}

/// Convert file to module
pub struct ResolveContext {
    pub(crate) package: Arc<str>,
    /// The current namespace
    pub(crate) namespace: Vec<Arc<str>>,
    /// The document buffer
    pub(crate) document: String,
    /// main function of the file
    pub(crate) unsolved: HashMap<Identifier, StatementNode>,
    /// The declared items in file
    pub(crate) items: IndexMap<Identifier, ModuleItem>,
    /// Collect errors
    pub(crate) errors: Vec<NyarError>,
    /// Collect spread statements
    pub(crate) main_function: Vec<StatementNode>,
}

pub enum ModuleItem {
    Resource(ValkyrieResource),
    External(ValkyrieExternalFunction),
    // Imported(ValkyrieSymbol),
    // Function(ValkyrieFunction),
    Structure(ValkyrieStructure),
    Variant(ValkyrieUnion),
}

impl ResolveContext {
    pub fn new<S: Into<Arc<str>>>(package: S) -> Self {
        Self {
            package: package.into(),
            namespace: vec![],
            document: "".to_string(),
            unsolved: Default::default(),
            items: Default::default(),
            errors: vec![],
            main_function: vec![],
        }
    }
}

impl ResolveContext {
    pub fn get_name_path(&self, symbol: &IdentifierNode) -> Identifier {
        let mut namespace = Vec::with_capacity(8);
        let name = Arc::from(symbol.name.as_str());
        if let Some(s) = &self.namespace {
            if let [head, rest @ ..] = s.path.as_slice() {
                match head.name.eq("package") {
                    true => namespace.push(self.package.clone()),
                    false => namespace.push(Arc::from(head.name.as_str())),
                }
                for x in rest {
                    namespace.push(Arc::from(x.name.as_str()))
                }
            }
        }
        Identifier { namespace, name }
    }
}

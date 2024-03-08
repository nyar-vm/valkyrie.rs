use crate::{
    helpers::{Hir2Mir, Mir2Lir},
    structures::ValkyrieClass,
    ValkyrieFunction, ValkyrieUnion,
};
use convert_case::{Case, Casing};
use im::HashMap;
use indexmap::IndexMap;
use nyar_error::{Failure, ForeignInterfaceError, NyarError, Result, SourceCache, SourceID, SourceSpan, Success};
use nyar_wasm::{CanonicalWasi, DependentGraph, Identifier, WasiImport, WasiModule};
use std::{
    fmt::{Debug, Formatter},
    mem::take,
    str::FromStr,
    sync::Arc,
};
use valkyrie_ast::{
    AnnotationNode, ArgumentTerm, IdentifierNode, NamespaceDeclaration, ProgramRoot, StatementKind, StringTextNode,
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
    pub(crate) using: HashMap<Identifier, Identifier>,
    /// The declared items in file
    pub(crate) items: IndexMap<Identifier, ModuleItem>,
    /// Collect errors
    errors: Vec<NyarError>,
    /// Collect spread statements
    pub(crate) main_function: Vec<StatementNode>,
}

pub enum ModuleItem {
    Function(ValkyrieFunction),
    // Imported(ValkyrieSymbol),
    // Function(ValkyrieFunction),
    Structure(ValkyrieClass),
    Variant(ValkyrieUnion),
}

impl ResolveContext {
    pub fn new<S: Into<Arc<str>>>(package: S) -> Self {
        Self {
            package: package.into(),
            namespace: vec![],
            document: "".to_string(),
            using: Default::default(),
            items: Default::default(),
            errors: vec![],
            main_function: vec![],
        }
    }
}

impl ResolveContext {
    pub fn push_error<E: Into<NyarError>>(&mut self, e: E) {
        self.errors.push(e.into())
    }

    /// Get the full name path based on package name and namespace, then register the name to local namespace.
    pub fn register_item(&mut self, symbol: &IdentifierNode) -> Identifier {
        let key = Identifier { namespace: vec![], name: symbol.name.clone() };
        let value = Identifier { namespace: self.namespace.clone(), name: symbol.name.clone() };
        match self.using.insert(key, value.clone()) {
            Some(_) => {
                unimplemented!("dup")
            }
            None => {}
        }
        value
    }
    pub fn get_foreign_module(
        &mut self,
        info: &AnnotationNode,
        kind: &'static str,
        hint: &'static str,
        keyword: SourceSpan,
    ) -> Option<(WasiModule, Arc<str>)> {
        let ffi = info.attributes.get("ffi")?;
        if !hint.is_empty() {
            if !info.modifiers.contains(hint) {
                self.push_error(ForeignInterfaceError::MissingForeignFlag { kind, hint, span: keyword });
            }
        }
        match ffi.get_ffi_modules() {
            Ok((module, name)) => match WasiModule::from_str(&module.text) {
                Ok(o) => return Some((o, name)),
                Err(e) => self.push_error(e.with_span(module.span.clone())),
            },
            Err(e) => self.push_error(e),
        }
        return None;
    }
    /// Get the full name path based on package name and namespace
    pub fn import_field(&self, symbol: &IdentifierNode, alias: &AnnotationNode) -> Result<(Arc<str>, Arc<str>)> {
        let wasi_alias = match alias.attributes.get("ffi").and_then(|x| x.arguments.terms.first()) {
            Some(s) => match s.value.as_text() {
                Some(s) => Arc::from(s.text.as_str()),
                None => Err(NyarError::custom("missing wasi alias"))?,
            },
            None => Arc::from(symbol.name.as_ref().to_case(Case::Kebab)),
        };
        Ok((symbol.name.clone(), wasi_alias))
    }

    /// Get the full name path based on package name and namespace
    pub fn import_function(&mut self, alias: &AnnotationNode, symbol: &IdentifierNode) -> Option<WasiImport> {
        let import = alias.attributes.get("import")?;
        let module = self.get_wasi_module(import.arguments.terms.get(0), import.span)?;
        let name: Arc<str> = match import.arguments.terms.get(1) {
            Some(term) => match term.value.as_text() {
                Some(node) => Arc::from(node.text.as_str()),
                None => {
                    self.push_error(ForeignInterfaceError::InvalidForeignName { span: term.span });
                    return None;
                }
            },
            None => Arc::from(symbol.name.as_ref().to_case(Case::Kebab)),
        };
        Some(WasiImport { module, name })
    }

    fn get_wasi_module(&mut self, term: Option<&ArgumentTerm>, span: SourceSpan) -> Option<WasiModule> {
        match term.and_then(|x| x.value.as_text()) {
            Some(text) => match WasiModule::from_str(&text.text) {
                Ok(o) => Some(o),
                Err(e) => {
                    self.push_error(e.with_span(text.span.clone()));
                    None
                }
            },
            None => {
                self.push_error(ForeignInterfaceError::InvalidForeignModule { span });
                None
            }
        }
    }
}

use crate::structures::ValkyrieStructure;
use indexmap::IndexMap;
use nyar_error::{Failure, FileCache, FileID, NyarError, Result, Success};
use nyar_wasm::Identifier;
use std::{
    fmt::{Debug, Formatter},
    mem::take,
    sync::Arc,
};
use valkyrie_ast::{IdentifierNode, NamePathNode, NamespaceDeclaration, ProgramRoot, StatementKind};
use valkyrie_parser::{ProgramContext, StatementNode};

pub struct ValkyrieModule {}

/// Convert file to module
#[derive(Debug, Default)]
pub struct ResolveContext {
    /// The current namespace
    pub(crate) namespace: Option<NamePathNode>,
    /// The document buffer
    pub(crate) document: String,
    /// main function of the file
    pub(crate) unsolved: Vec<StatementNode>,
    /// The declared items in file
    pub(crate) items: IndexMap<Identifier, ModuleItem>,
    /// Collect errors
    pub(crate) errors: Vec<NyarError>,
}

pub enum ModuleItem {
    // External(ValkyrieExternalFunction),
    // Imported(ValkyrieSymbol),
    // Function(ValkyrieFunction),
    Structure(ValkyrieStructure),
}

impl Debug for ModuleItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // Self::External(v) => Debug::fmt(v, f),
            // Self::Imported(v) => Debug::fmt(v, f),
            Self::Structure(v) => Debug::fmt(v, f),
            // Self::Function(v) => Debug::fmt(v, f),
        }
    }
}

pub(crate) trait Hir2Mir {
    type Output = ();
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output>;
}

pub(crate) trait Mir2Lir {
    type Output = ();
    fn to_lir(self, ctx: &mut ResolveContext) -> Result<Self::Output>;
}

impl Hir2Mir for ProgramRoot {
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        for statement in self.statements {
            statement.to_mir(ctx)?
        }
        Ok(())
    }
}

impl Hir2Mir for StatementKind {
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        match self {
            Self::Nothing => {}
            Self::Document(_) => {}
            Self::Annotation(_) => {}
            Self::Namespace(v) => v.to_mir(ctx)?,
            Self::Import(_) => {}
            Self::Class(v) => {}
            Self::Union(_) => {}
            Self::Enumerate(_) => {}
            Self::Trait(_) => {}
            Self::Extends(_) => {}
            Self::Function(f) => {}
            Self::Variable(_) => {}
            Self::Guard(_) => {}
            Self::While(_) => {}
            Self::For(_) => {}
            Self::Control(_) => {}
            Self::Expression(_) => {}
        }
        Ok(())
    }
}

impl Hir2Mir for NamespaceDeclaration {
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        // ctx.namespace = Some(self.path.as_symbol());
        Ok(())
    }
}
impl ResolveContext {
    pub fn parse(&mut self, file: FileID, cache: &mut FileCache) -> Vec<NyarError> {
        let root = ProgramContext { file }.parse(cache);
        let mut errors = vec![];
        match root {
            Success { value, diagnostics } => {
                errors.extend(diagnostics);
                errors.extend(self.visit(value))
            }
            Failure { fatal, diagnostics } => {
                errors.extend(diagnostics);
                errors.extend_one(fatal);
            }
        }
        errors
    }

    pub fn visit(&mut self, root: ProgramRoot) -> Vec<NyarError> {
        let progress = root.to_mir(self);
        let mut errors = take(&mut self.errors);
        match progress {
            Ok(_) => {}
            Err(e) => errors.push(e),
        }
        errors
    }
}

impl ResolveContext {
    pub fn get_name_path(&self, symbol: &IdentifierNode) -> Identifier {
        let name = Arc::from(symbol.name.as_str());
        match &self.namespace {
            Some(s) => Identifier { namespace: s.path.iter().map(|x| Arc::from(x.name.as_str())).collect(), name },
            None => Identifier { namespace: vec![], name },
        }
    }
}

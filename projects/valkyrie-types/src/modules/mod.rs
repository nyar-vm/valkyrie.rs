use crate::{values::symbols::AsSymbol, FileCache, FileID, ValkyrieStructure, ValkyrieSymbol};
use indexmap::{map::Entry, IndexMap};
use nyar_error::{Failure, NyarError, Result, Success, Validation};
use nyar_wasm::Operation;
use std::{
    fmt::{Debug, Formatter},
    mem::take,
};
use valkyrie_ast::{ClassDeclaration, NamespaceDeclaration, ProgramRoot, StatementKind};
use valkyrie_parser::{ProgramContext, StatementNode};

pub struct ValkyrieModule {}

/// Convert file to module
#[derive(Debug, Default)]
pub struct ModuleResolver {
    /// The declared namespace
    pub(crate) namespace: Option<ValkyrieSymbol>,
    /// main function of the file
    pub(crate) main: Vec<StatementNode>,
    /// The declared items in file
    pub(crate) items: IndexMap<String, ModuleItem>,
    /// Collect errors
    pub(crate) errors: Vec<NyarError>,
}

pub enum ModuleItem {
    External(ValkyrieSymbol),
    Imported(ValkyrieSymbol),
    Structure(ValkyrieStructure),
}

impl Debug for ModuleItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::External(v) => Debug::fmt(v, f),
            Self::Imported(v) => Debug::fmt(v, f),
            Self::Structure(v) => Debug::fmt(v, f),
        }
    }
}

pub(crate) trait AsModuleItem {
    type Output = ();
    fn send_module(self, ctx: &mut ModuleResolver) -> Result<Self::Output>;
}

impl AsModuleItem for ProgramRoot {
    fn send_module(self, ctx: &mut ModuleResolver) -> Result<Self::Output> {
        for statement in self.statements {
            statement.send_module(ctx)?
        }
        Ok(())
    }
}

impl AsModuleItem for StatementKind {
    fn send_module(self, ctx: &mut ModuleResolver) -> Result<Self::Output> {
        match self {
            Self::Nothing => {}
            Self::Document(_) => {}
            Self::Annotation(_) => {}
            Self::Namespace(v) => v.send_module(ctx)?,
            Self::Import(_) => {}
            Self::Class(v) => v.send_module(ctx)?,
            Self::Union(_) => {}
            Self::Enumerate(_) => {}
            Self::Trait(_) => {}
            Self::Extends(_) => {}
            Self::Function(_) => {}
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

impl AsModuleItem for NamespaceDeclaration {
    fn send_module(self, ctx: &mut ModuleResolver) -> Result<Self::Output> {
        ctx.namespace = Some(self.path.as_symbol());
        Ok(())
    }
}
impl AsModuleItem for ClassDeclaration {
    fn send_module(self, ctx: &mut ModuleResolver) -> Result<Self::Output> {
        let symbol = self.name.as_namespace_symbol(&ctx.namespace);
        let item = ValkyrieStructure { symbol, fields: Default::default(), methods: Default::default() };
        match ctx.items.entry(item.name()) {
            Entry::Occupied(e) => {
                todo!()
            }
            Entry::Vacant(e) => {
                e.insert(ModuleItem::Structure(item));
            }
        }
        Ok(())
    }
}

impl ModuleResolver {
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
        let progress = root.send_module(self);
        let mut errors = take(&mut self.errors);
        match progress {
            Ok(_) => {}
            Err(e) => errors.push(e),
        }
        errors
    }
}

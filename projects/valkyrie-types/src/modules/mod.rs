use crate::{ValkyrieStructure, ValkyrieSymbol};
use indexmap::IndexMap;
use nyar_error::{NyarError, Result};
use nyar_wasm::Operation;
use std::mem::take;
use valkyrie_ast::{NamespaceDeclaration, ProgramRoot, StatementKind};
use valkyrie_parser::StatementNode;

pub struct ValkyrieModule {}

/// Convert file to module
pub struct ModuleContext {
    /// The declared namespace
    namespace: Option<ValkyrieSymbol>,
    /// main function of the file
    main: Vec<StatementNode>,
    /// The declared items in file
    items: IndexMap<String, ModuleItem>,
    /// Collect errors
    errors: Vec<NyarError>,
}

pub enum ModuleItem {
    Structure(ValkyrieStructure),
}

trait AsModuleItem {
    type Output = ();
    fn send_module(self, ctx: &mut ModuleContext) -> Result<Self::Output>;
}

impl AsModuleItem for ProgramRoot {
    fn send_module(self, ctx: &mut ModuleContext) -> Result<Self::Output> {
        for statement in self.statements {
            statement.send_module(ctx)?
        }
        Ok(())
    }
}

impl AsModuleItem for StatementKind {
    fn send_module(self, ctx: &mut ModuleContext) -> Result<Self::Output> {
        match self {
            Self::Nothing => {}
            Self::Document(_) => {}
            Self::Annotation(_) => {}
            Self::Namespace(v) => v.send_module(ctx)?,
            Self::Import(_) => {}
            Self::Class(_) => {}
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
    fn send_module(self, ctx: &mut ModuleContext) -> Result<Self::Output> {
        todo!()
    }
}

impl ModuleContext {
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

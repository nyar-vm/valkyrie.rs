use crate::{helpers::Hir2Mir, structures::ValkyrieStructure, ValkyrieExternalFunction, ValkyrieUnion, ValkyrieUnionItem};
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

pub struct ValkyrieModule {}

/// Convert file to module
#[derive(Default)]
pub struct ResolveContext {
    /// The current namespace
    pub(crate) namespace: Option<NamePathNode>,
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

impl Debug for ResolveContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let namespace = match &self.namespace {
            None => Cow::Borrowed("None"),
            Some(s) => Cow::Owned(s.to_string()),
        };
        f.debug_struct("ResolveContext")
            .field("namespace", &namespace)
            .field("document", &self.document)
            .field("unsolved", &self.unsolved)
            .field("items", &self.items)
            .field("errors", &self.errors)
            .field("main_function", &self.main_function)
            .finish()
    }
}

pub enum ModuleItem {
    External(ValkyrieExternalFunction),
    // Imported(ValkyrieSymbol),
    // Function(ValkyrieFunction),
    Structure(ValkyrieStructure),
    Variant(ValkyrieUnion),
}

impl Debug for ModuleItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::External(v) => Debug::fmt(v, f),
            // Self::Imported(v) => Debug::fmt(v, f),
            Self::Structure(v) => Debug::fmt(v, f),
            Self::Variant(v) => Debug::fmt(v, f),
            // Self::Function(v) => Debug::fmt(v, f),
        }
    }
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
            Self::Nothing => {
                todo!()
            }
            Self::Document(_) => {
                todo!()
            }
            Self::Annotation(_) => {
                todo!()
            }
            Self::Namespace(v) => v.to_mir(ctx)?,
            Self::Import(_) => {
                todo!()
            }
            Self::Class(v) => v.to_mir(ctx)?,
            Self::Union(v) => {
                let variant = v.to_mir(ctx)?;
                ctx.items.insert(variant.symbol.clone(), ModuleItem::Variant(variant));
            }
            Self::Enumerate(_) => {
                todo!()
            }
            Self::Trait(_) => {
                todo!()
            }
            Self::Extends(_) => {
                todo!()
            }
            Self::Function(v) => {
                let fun = v.to_mir(ctx)?;
            }
            Self::Variable(_) => {
                todo!()
            }
            Self::Guard(_) => {
                todo!()
            }
            Self::While(_) => {
                todo!()
            }
            Self::For(_) => {
                todo!()
            }
            Self::Control(_) => {
                todo!()
            }
            Self::Expression(_) => {
                todo!()
            }
        }
        Ok(())
    }
}

impl Hir2Mir for FunctionDeclaration {
    type Output = ModuleItem;
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        Ok(ModuleItem::External(ValkyrieExternalFunction {}))
    }
}

impl Hir2Mir for NamespaceDeclaration {
    fn to_mir(self, ctx: &mut ResolveContext) -> Result<Self::Output> {
        ctx.namespace = Some(self.path);
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

    pub fn resolve(&self) -> Result<CanonicalWasi> {
        let mut output = DependentGraph::default();
        {
            for item in self.items.values() {
                match item {
                    ModuleItem::Structure(s) => match &s.external_resource {
                        Some(s) => output += s.clone(),
                        None => {}
                    },
                    ModuleItem::Variant(_) => {}
                    ModuleItem::External(_) => {}
                }
            }
        }
        Ok(CanonicalWasi::new(output)?)
    }
}

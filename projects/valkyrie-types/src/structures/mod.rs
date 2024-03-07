use crate::{
    helpers::Hir2Mir,
    modules::{ModuleItem, ResolveContext},
};
use indexmap::{
    map::{Entry, Values},
    IndexMap,
};
use nyar_error::{NyarError, Result};
use nyar_wasm::{Identifier, WasiModule, WasiRecordField, WasiRecordType, WasiResource};
use std::{
    fmt::{Debug, Formatter},
    ops::AddAssign,
    sync::Arc,
};
use valkyrie_ast::{helper::WrapDisplay, ClassDeclaration, ClassTerm, FieldDeclaration, IdentifierNode, NamePathNode};

// mod codegen;
mod parser;

#[derive(Clone, Eq, PartialEq)]
pub struct ValkyrieStructure {
    pub(crate) symbol: Identifier,
    pub(crate) fields: IndexMap<Arc<str>, ValkyrieField>,
    pub(crate) methods: IndexMap<String, ValkyrieMethod>,
    /// Whether the class is an external resource type
    pub(crate) external_resource: Option<WasiResource>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ValkyrieResource {
    pub(crate) symbol: Identifier,
    pub(crate) wasi_module: WasiModule,
    pub(crate) wasi_name: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValkyrieField {
    /// The name of the field
    pub name: Arc<str>,
    /// The WASI name of the field
    pub wasi_name: Arc<str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValkyrieMethod {}

impl Debug for ValkyrieStructure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug = &mut f.debug_struct("Structure");
        debug.field("symbol", &WrapDisplay::new(&self.symbol)).field("fields", &self.fields.values());
        if let Some(s) = &self.external_resource {
            debug.field("resource", s);
        }
        debug.finish()
    }
}

impl AddAssign<ValkyrieField> for ValkyrieStructure {
    fn add_assign(&mut self, rhs: ValkyrieField) {
        self.fields.insert(rhs.name.clone(), rhs);
    }
}
//
// impl AddAssign<MethodDefinition> for ValkyrieStructure {
//     fn add_assign(&mut self, rhs: MethodDefinition) {
//         self.methods.insert(rhs.name(), rhs);
//     }
// }

impl ValkyrieStructure {
    pub fn new(symbol: Identifier) -> Self {
        Self { symbol, fields: Default::default(), methods: Default::default(), external_resource: None }
    }
    pub fn get_name(&self) -> String {
        self.symbol.to_string()
    }
    // pub fn get_field(&self, name: &str) -> Option<&ValkyrieField> {
    //     self.fields.get(name)
    // }
    // pub fn add_field(&mut self, field: ValkyrieField) -> Result<()> {
    //     let name = field.name();
    //     let span = field.get_span();
    //     match self.fields.insert(field.name(), field) {
    //         Some(old) => Err(NyarError::duplicate_key(name, old.get_span(), span)),
    //         None => Ok(()),
    //     }
    // }
    // pub fn get_fields(&self) -> Values<String, ValkyrieField> {
    //     self.fields.values()
    // }
    // pub fn add_method(&mut self, method: MethodDefinition) -> Result<()> {
    //     let name = method.name();
    //     let span = method.get_span();
    //     match self.methods.insert(method.name(), method) {
    //         Some(old) => Err(NyarError::duplicate_key(name, old.get_span(), span)),
    //         None => Ok(()),
    //     }
    // }
}

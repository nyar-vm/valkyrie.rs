use crate::{
    backends::ConvertTo, modules::HIR, types::method_type::MethodDefinition, values::symbols::AsSymbol, FieldDefinition,
    ModuleItem, ModuleResolver, ValkyrieSymbol,
};
use indexmap::{
    map::{Entry, Values},
    IndexMap,
};
use nyar_error::{NyarError, Result};
use nyar_wasm::StructureType;
use std::{
    fmt::{Debug, Formatter},
    ops::AddAssign,
};
use valkyrie_ast::{helper::WrapDisplay, ClassDeclaration, ClassTerm, IdentifierNode, NamePathNode};

mod codegen;
mod parser;

#[derive(Clone, Eq, PartialEq)]
pub struct ValkyrieStructure {
    pub(crate) symbol: ValkyrieSymbol,
    pub(crate) fields: IndexMap<String, FieldDefinition>,
    pub(crate) methods: IndexMap<String, MethodDefinition>,
}

impl Debug for ValkyrieStructure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Structure")
            .field("symbol", &WrapDisplay::new(&self.symbol))
            .field("fields", &self.fields)
            .field("methods", &self.methods)
            .finish()
    }
}

impl AddAssign<FieldDefinition> for ValkyrieStructure {
    fn add_assign(&mut self, rhs: FieldDefinition) {
        self.fields.insert(rhs.name(), rhs);
    }
}

impl AddAssign<MethodDefinition> for ValkyrieStructure {
    fn add_assign(&mut self, rhs: MethodDefinition) {
        self.methods.insert(rhs.name(), rhs);
    }
}

impl ValkyrieStructure {
    pub fn new(space: &NamePathNode, name: &IdentifierNode) -> Self {
        todo!()
    }
    pub fn name(&self) -> String {
        self.symbol.to_string()
    }
    pub fn get_field(&self, name: &str) -> Option<&FieldDefinition> {
        self.fields.get(name)
    }
    pub fn add_field(&mut self, field: FieldDefinition) -> Result<()> {
        let name = field.name();
        let span = field.get_span();
        match self.fields.insert(field.name(), field) {
            Some(old) => Err(NyarError::duplicate_key(name, old.get_span(), span)),
            None => Ok(()),
        }
    }
    pub fn get_fields(&self) -> Values<String, FieldDefinition> {
        self.fields.values()
    }
    pub fn add_method(&mut self, method: MethodDefinition) -> Result<()> {
        let name = method.name();
        let span = method.get_span();
        match self.methods.insert(method.name(), method) {
            Some(old) => Err(NyarError::duplicate_key(name, old.get_span(), span)),
            None => Ok(()),
        }
    }
}

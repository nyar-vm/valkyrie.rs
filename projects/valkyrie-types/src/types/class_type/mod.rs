use super::*;
use crate::types::method_type::MethodDefinition;
use indexmap::map::Values;
use nyar_error::{DuplicateError, DuplicateKind, NyarError};
use std::ops::AddAssign;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassDefinition {
    symbol: Vec<Arc<str>>,
    fields: IndexMap<String, FieldDefinition>,
    methods: IndexMap<String, MethodDefinition>,
    span: FileSpan,
}

impl Hash for ClassDefinition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
        for (k, v) in self.fields.iter() {
            k.hash(state);
        }
    }
}

impl AddAssign<FieldDefinition> for ClassDefinition {
    fn add_assign(&mut self, rhs: FieldDefinition) {
        self.fields.insert(rhs.name(), rhs);
    }
}
impl AddAssign<MethodDefinition> for ClassDefinition {
    fn add_assign(&mut self, rhs: MethodDefinition) {
        self.methods.insert(rhs.name(), rhs);
    }
}

impl FromFrontend<ClassDefinition> for ClassDeclaration {
    fn build(&self, state: &mut ValkyrieCodegen) -> nyar_error::Result<ClassDefinition> {
        let mut output = ClassDefinition::new(&state.current_namespace, &self.name);
        for x in &self.terms {
            match x {
                ClassTerm::Macro(_) => {}
                ClassTerm::Field(v) => match output.add_field(v.build(state)?) {
                    Err(e) if !state.interactive => state.add_error(e),
                    _ => {}
                },
                ClassTerm::Method(v) => match output.add_method(v.build(state)?) {
                    Err(e) if !state.interactive => state.add_error(e),
                    _ => {}
                },
                ClassTerm::Domain(_) => {}
            }
        }
        Ok(output)
    }
}
impl ClassDefinition {
    pub fn new(space: &NamePathNode, name: &IdentifierNode) -> Self {
        let mut symbol = Vec::with_capacity(space.path.len() + 1);
        symbol.extend(space.path.iter().map(|s| Arc::from(s.name.as_str())));
        symbol.extend_one(Arc::from(name.name.as_str()));
        Self { symbol, fields: Default::default(), methods: Default::default(), span: Default::default() }
    }
    pub fn name(&self) -> String {
        self.symbol.join("∷")
    }
    pub fn get_field(&self, name: &str) -> Option<&FieldDefinition> {
        self.fields.get(name)
    }
    pub fn add_field(&mut self, field: FieldDefinition) -> Result<(), NyarError> {
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
    pub fn add_method(&mut self, method: MethodDefinition) -> Result<(), NyarError> {
        let name = method.name();
        let span = method.get_span();
        match self.methods.insert(method.name(), method) {
            Some(old) => Err(NyarError::duplicate_key(name, old.get_span(), span)),
            None => Ok(()),
        }
    }
}

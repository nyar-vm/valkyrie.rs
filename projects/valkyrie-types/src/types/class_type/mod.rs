use super::*;
use crate::{helpers::FromFrontend, ValkyrieCodegen};
use valkyrie_ast::{ClassDeclaration, ClassTerm};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassDefinition {
    symbol: Vec<Arc<str>>,
    items: IndexMap<String, ValkyrieValue>,
    span: FileSpan,
}

impl Hash for ClassDefinition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
        for (k, v) in self.items.iter() {
            k.hash(state);
            v.hash(state)
        }
    }
}

impl ClassDefinition {
    pub fn new(space: &NamePathNode, name: &IdentifierNode) -> Self {
        let mut symbol = Vec::with_capacity(space.path.len() + 1);
        symbol.extend(space.path.iter().map(|s| Arc::from(s.name.as_str())));
        symbol.extend_one(Arc::from(name.name.as_str()));
        Self { symbol, items: Default::default(), span: Default::default() }
    }
    pub fn name(&self) -> String {
        self.symbol.join("∷")
    }
    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn extend_many<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = ValkyrieValue>,
    {
        todo!()
    }

    pub fn extend_one(&mut self, item: ValkyrieValue) {
        todo!()
    }
}

impl Default for ClassDefinition {
    fn default() -> Self {
        todo!()
    }
}

impl FromFrontend<ClassDefinition> for ClassDeclaration {
    fn build(&self, state: &mut ValkyrieCodegen) -> nyar_error::Result<ClassDefinition> {
        let output = ClassDefinition::new(&state.current_namespace, &self.name);
        for x in self.terms {
            match x {
                ClassTerm::Macro(_) => {}
                ClassTerm::Field(v) => {}
                ClassTerm::Method(_) => {}
                ClassTerm::Domain(_) => {}
            }
        }
        Ok(output)
    }
}

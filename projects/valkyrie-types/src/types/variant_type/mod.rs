use super::*;

#[derive(Clone, Debug, Scan)]
pub struct ValkyrieVariantType {
    namepath: Vec<String>,
    generics: Vec<Gc<ValkyrieMetaType>>,
    variants: Vec<Arc<ValkyrieClassType>>,
}

impl ValkyrieVariantType {
    pub fn new(namepath: String) -> Self {
        Self { namepath: namepath.split('.').map(|s| s.to_string()).collect(), generics: vec![], variants: vec![] }
    }
    pub fn mut_generics(&mut self) -> &mut Vec<Gc<ValkyrieMetaType>> {
        &mut self.generics
    }
}

impl Default for ValkyrieVariantType {
    fn default() -> Self {
        todo!()
    }
}

impl ValkyrieType for ValkyrieVariantType {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Variant(Gc::new(self))
    }

    fn dynamic_type(&self) -> Gc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        *this.mut_namepath() = self.namepath.clone();
        this.mut_generic_types().extend(self.generics.iter().cloned());
        Gc::new(this)
    }
}

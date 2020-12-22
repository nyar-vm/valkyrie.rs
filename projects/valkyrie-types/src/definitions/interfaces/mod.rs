use super::*;

pub struct ValkyrieInterface {
    /// package∷module::Interface
    namepath: String,
    /// trait or interfaces
    is_trait: bool,
    /// The functions that are defined in this interfaces
    document: ValkyrieDocument,
}

impl Debug for ValkyrieInterface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let kind = match self.is_trait {
            true => "ValkyrieTrait",
            false => "ValkyrieInterface",
        };
        let f = &mut f.debug_struct(kind);
        f.field("name", &self.namepath);
        f.field("document", &self.document);
        f.finish()
    }
}

impl ValkyrieInterface {
    pub fn new(namepath: String) -> Self {
        Self { namepath, is_trait: false, document: Default::default() }
    }
    pub fn mark_trait(self, is_trait: bool) -> Self {
        Self { is_trait, ..self }
    }
    pub fn with_file(self, file: String) -> Self {
        todo!()
    }
    pub fn with_span(self, span: Range<u32>) -> Self {
        todo!()
    }

    pub fn is_trait(&self) -> bool {
        self.is_trait
    }
    pub fn is_interface(&self) -> bool {
        !self.is_trait
    }
    pub fn namespace(&self) -> &str {
        match self.namepath.rsplit_once("∷") {
            Some((namespace, _)) => namespace,
            None => "",
        }
    }
    pub fn name(&self) -> &str {
        match self.namepath.rsplit_once("∷") {
            Some((_, name)) => name,
            None => self.namepath.as_str(),
        }
    }
    pub fn full_name(&self) -> &str {
        self.namepath.as_str()
    }
}

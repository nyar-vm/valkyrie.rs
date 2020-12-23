use super::*;

#[derive(Clone, Debug)]
pub struct ValkyrieStructure {
    /// package∷module::Interface
    namepath: ValkyrieID,
    auto_traits: Vec<ValkyrieID>,
    imply_traits: Vec<ValkyrieID>,
    super_classes: Vec<ValkyrieID>,
    fields: BTreeMap<String, ValkyrieField>,
    /// fields are also properties
    properties: BTreeMap<String, ValkyrieProperty>,
}

#[derive(Clone, Debug)]
pub struct ValkyrieField {
    name: String,
    access: AccessType,
    typing: ValkyrieMetaType,
    /// Can be a progress or a value
    default: Option<ExpressionNode>,
}

/// A property is a field that can be accessed by a getter and setter
#[derive(Clone, Debug)]
pub struct ValkyrieProperty {
    name: String,
    initial: InitializeType,
    typing: ValkyrieMetaType,
    /// Can be a progress or a value
    default: Option<ExpressionNode>,
}

impl ValkyrieStructure {
    pub fn new(namepath: ValkyrieID) -> Self {
        Self {
            namepath,
            auto_traits: vec![],
            imply_traits: vec![],
            super_classes: vec![],
            fields: Default::default(),
            properties: Default::default(),
        }
    }
    pub fn get_name(&self) -> &str {
        self.namepath.name()
    }
    pub fn get_namespace(&self) -> &[String] {
        self.namepath.namespace()
    }
    pub fn get_namepath(&self) -> &[String] {
        self.namepath.full_name()
    }
    pub fn has_trait(&self, id: &ValkyrieID) -> bool {
        self.auto_traits.contains(id) || self.imply_traits.contains(id)
    }
    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }
}

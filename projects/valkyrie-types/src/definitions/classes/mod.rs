pub struct ValkyrieStructure {
    /// package∷module::Interface
    namepath: ValkyrieNamepath,
}

impl ValkyrieStructure {
    pub fn new(namepath: ValkyrieNamepath) -> Self {
        Self { namepath }
    }
}

pub struct ValkyrieNamepath {
    inner: Vec<String>,
}

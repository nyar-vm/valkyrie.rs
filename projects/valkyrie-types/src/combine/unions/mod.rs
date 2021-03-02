use crate::{ValkyrieID, ValkyrieType};
use indexmap::IndexMap;
use std::collections::BTreeSet;

pub struct ValkyrieUnion {
    structured: Option<ValkyrieID>,
    set: IndexMap<String, ValkyrieType>,
}

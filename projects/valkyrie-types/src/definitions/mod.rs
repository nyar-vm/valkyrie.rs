use crate::{
    modifiers::AccessType,
    types::{atomic_type::ValkyrieDocument, ValkyrieMetaType},
    InitializeType, ValkyrieEnumerate, ValkyrieError, ValkyrieID,
};
use nyar_error::{Success, Validation};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    ops::Range,
};
use valkyrie_ast::{ExpressionNode, FlagDeclaration, FlagKind, IdentifierNode, NamePathNode};

pub mod classes;
pub mod enumerates;
pub mod fields;
pub mod ids;
pub mod instances;
pub mod interfaces;
pub mod names;

pub struct Valhalla {
    /// Current working namespace
    scope: ValkyrieID,
    items: BTreeMap<String, ValhallaItem>,
}

pub struct ValhallaBuilder {}

pub enum ValhallaItem {
    Enumerate(ValkyrieEnumerate),
}
impl Valhalla {
    pub(crate) fn load_ns(&self, o: NamePathNode) -> Result<ValkyrieID, ValkyrieError> {
        todo!()
    }
    pub(crate) fn load_id(&self, o: IdentifierNode) -> ValkyrieID {
        let mut path = self.scope.path.clone();
        path.push(o.name);
        ValkyrieID { path, location: o.span }
    }
}

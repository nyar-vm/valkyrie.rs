use crate::{
    modifiers::AccessType,
    types::{atomic_type::ValkyrieDocument, ValkyrieMetaType},
    InitializeType, ValkyrieID, ValkyrieValue,
};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    ops::Range,
};
use valkyrie_ast::ExpressionNode;
pub mod classes;
pub mod id;
pub mod interfaces;

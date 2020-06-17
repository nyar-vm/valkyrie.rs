#![feature(trivial_bounds)]

pub use self::{
    builtin::result::{ValkyrieFailure, ValkyrieSuccess},
    types::{
        atomic_type::ValkyrieAtomicType, class_type::ValkyrieClassType, literal_type::ValkyrieLiteralType,
        tuple_type::ValkyrieTable, union_type::ValkyrieUnionType, variant_type::ValkyrieVariantType, ValkyrieType,
    },
    values::ValkyrieValue,
};

pub mod third_party;

mod builtin;
mod codegen;
mod errors;
mod functions;

mod types;
mod values;
// #[cfg(test)]
pub mod testing;
mod utils;
pub use crate::{
    errors::{ValkyrieError, ValkyrieErrorKind, ValkyrieResult},
    functions::operators::ValkyrieOperator,
};

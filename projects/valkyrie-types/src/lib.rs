#![feature(trivial_bounds)]

mod builtin;
mod codegen;
mod functions;
mod modifiers;
// #[cfg(test)]
pub mod testing;
pub mod third_party;
mod traits;
mod types;
mod utils;
mod values;
mod validation;

pub use self::{
    builtin::{
        images::ValkyrieImage,
        result::{ValkyrieFailure, ValkyrieSuccess},
    },
    functions::{ValkyrieFunction, ValkyrieFunctionType, ValkyrieMonomorphicFunction},
    types::{
        atomic_type::ValkyrieAtomicType, class_type::ValkyrieClassType, literal_type::ValkyrieLiteralType,
        tuple_type::ValkyrieTable, union_type::ValkyrieUnionType, variant_type::ValkyrieVariantType, ValkyrieType,
    },
    values::ValkyrieValue,
};
pub use valkyrie_ast::ValkyrieOperator;
pub use valkyrie_error::{JsonValue, RuntimeError, SyntaxError, ValkyrieError, ValkyrieResult};

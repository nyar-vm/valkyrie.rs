#![feature(trivial_bounds)]
#![feature(allocator_api)]
#![feature(never_type)]
#![feature(unboxed_closures)]
#![feature(iter_from_generator)]
#![feature(generators)]
#![feature(lazy_cell)]
#![feature(extend_one)]

extern crate salsa_2022 as salsa;

pub mod helpers;
pub mod projects;

mod builtin;
mod codegen;
mod collection;
mod functions;
mod modifiers;
// #[cfg(test)]
mod definitions;
mod files;
mod packages;
pub mod testing;
pub mod third_party;
mod types;
mod utils;

mod values;

pub use self::{
    builtin::{
        images::ValkyrieImage,
        result::{ValkyrieFailure, ValkyrieSuccess},
        texts::{StringID, ValkyrieString},
        TokenType,
    },
    codegen::ValkyrieCodegen,
    collection::{
        dict::ValkyrieDict,
        list::{ValkyrieList, ValkyrieOrdinal},
    },
    definitions::{enumerates::ValkyrieEnumerate, interfaces::ValkyrieInterface, names::ValkyrieName},
    functions::{ValkyrieFunction, ValkyrieFunctionType, ValkyrieMonomorphicFunction},
    modifiers::{FeatureType, InitializeType, MutableType},
    packages::ids::{ValkyrieID, ValkyrieUniverse},
    types::{
        atomic_type::ValkyrieAtomicType, class_type::ValkyrieStructure, literal_type::ValkyrieLiteralType,
        union_type::ValkyrieUnionType, variant_type::ValkyrieVariantType, ValkyrieType,
    },
    values::ValkyrieValue,
};
pub use nyar_error::{
    Failure, FileCache, FileID, MissingError, NyarError as ValkyrieError, Result as ValkyrieResult, RuntimeError, Success,
    SyntaxError,
};
pub use nyar_number::{Num, NyarReal as ValkyrieNumber, One, Zero};
pub use shredder::Gc;
pub use valkyrie_ast::ValkyrieOperator;
pub use valkyrie_parser::{ProgramContext, StringFormatterBuilder};

mod jars;

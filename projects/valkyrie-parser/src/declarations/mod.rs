mod classes;
mod def_fn;
mod enumerate;
mod interface;
mod unions;

mod def_var;
mod extends;

use crate::{
    helpers::ProgramState,
    utils::{build_annotation_terms, build_modifier_ahead, build_type_hint},
    FunctionParametersNode,
};

use nyar_error::Result;
use valkyrie_ast::*;

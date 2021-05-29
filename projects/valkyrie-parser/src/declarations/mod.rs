mod classes;
mod def_fn;
mod enumerate;
mod interface;
mod unions;

mod def_var;
mod extends;

use crate::{helpers::ProgramState, FlagTermNode};
use nyar_error::{Success, Validate, Validation};
use valkyrie_ast::*;

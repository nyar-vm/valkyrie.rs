mod classes;
mod enumerate;
mod function;
mod interface;
mod unions;

mod def_var;
mod extends;

use crate::{helpers::ProgramState, ContinuationNode, ProgramContext};
use nyar_error::{Success, Validate, Validation};
use valkyrie_ast::*;

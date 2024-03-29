use crate::helpers::ProgramState;
use nyar_error::Result;
use valkyrie_ast::{ExpressionKind, *};

mod controller;
mod jmp_if;
mod jmp_match;
mod jmp_switch;
mod loop_for;
mod loop_while;

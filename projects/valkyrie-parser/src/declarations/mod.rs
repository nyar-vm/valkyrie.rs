mod annotation;
mod classes;
mod enumerate;
mod function;
mod interface;
mod unions;

mod extends;

use crate::{
    DefineExtendsNode, KwClassNode, KwFlagsNode, KwFunctionNode, KwTraitNode, KwUnionNode, ModifierCallNode, ProgramContext,
};
use nyar_error::{Success, Validation};
use valkyrie_ast::*;

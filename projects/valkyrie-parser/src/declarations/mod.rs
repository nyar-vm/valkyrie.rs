mod classes;
mod enumerate;
mod function;
mod interface;
mod unions;

use crate::{
    utils::build_modifiers, KwClassNode, KwFlagsNode, KwFunctionNode, KwTraitNode, KwUnionNode, ModifierCallNode,
    ProgramContext,
};
use nyar_error::{Success, Validation};
use valkyrie_ast::{
    ClassDeclaration, ClassKind, FlagDeclaration, FlagsKind, FunctionDeclaration, FunctionType, IdentifierNode, ModifiersNode,
    NamePathNode, TraitDeclaration, UnionDeclaration,
};

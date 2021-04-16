mod annotation;
mod classes;
mod enumerate;
mod function;
mod interface;
mod unions;

use crate::{KwClassNode, KwFlagsNode, KwFunctionNode, KwTraitNode, KwUnionNode, ModifierCallNode, ProgramContext};
use nyar_error::{Success, Validation};
use valkyrie_ast::{
    ClassDeclaration, ClassKind, FlagDeclaration, FlagsKind, FunctionDeclaration, FunctionType, IdentifierNode, ModifiersNode,
    NamePathNode, TraitDeclaration, UnionDeclaration,
};

mod classes;
mod enumerate;

use crate::{utils::build_modifiers, KwClassNode, KwFlagsNode, ModifierCallNode, ProgramContext};
use nyar_error::{Success, Validation};
use valkyrie_ast::{ClassDeclaration, ClassKind, FlagDeclaration, FlagsKind, ModifiersNode};

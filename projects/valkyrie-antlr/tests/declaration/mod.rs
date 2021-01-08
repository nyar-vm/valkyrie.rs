use super::*;
use valkyrie_ast::GenericArgument;

#[test]
fn antlr_declarations() {
    top_debug(include_str!("namespaces.vk"), "declaration/namespaces_debug.rkt").expect("namespaces");
    top_debug(include_str!("class.vk"), "declaration/class_debug.rkt").expect("class");
    top_debug(include_str!("flags.vk"), "declaration/flags_debug.rkt").expect("flags");
    top_debug(include_str!("unions.vk"), "declaration/union_debug.rkt").expect("unions");
    // top_debug(include_str!("function.vk"), "declaration/function_debug.rkt").expect("function");
}

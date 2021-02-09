use super::*;
use valkyrie_ast::GenericArgument;

#[test]
fn antlr_declarations() {
    top_debug(include_str!("class.vk"), "declaration/class_debug.rkt").expect("class");
    top_debug(include_str!("flags.vk"), "declaration/flags_debug.rkt").expect("flags");
    // top_debug(include_str!("function.vk"), "declaration/function_debug.rkt").expect("function");
    top_debug(include_str!("let.vk"), "declaration/let_debug.rkt").expect("let");
    // top_debug(include_str!("macro.vk"), "declaration/macro_debug.rkt").expect("macro");
    top_debug(include_str!("namespaces.vk"), "declaration/namespaces.ron").expect("namespaces");
    top_debug(include_str!("trait.vk"), "declaration/trait_debug.rkt").expect("trait");
    // top_debug(include_str!("type.vk"), "declaration/type_debug.rkt").expect("type");
    top_debug(include_str!("union.vk"), "declaration/union_debug.rkt").expect("unions");
}

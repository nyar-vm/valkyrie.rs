use super::*;
use valkyrie_ast::GenericArgument;

#[test]
fn lex_declarations() {
    top_debug(include_str!("namespaces.vk"), "declaration/namespaces_debug.rkt").expect("namespaces");
    top_debug(include_str!("class.vk"), "declaration/class_debug.rkt").expect("class");
    // top_debug(include_str!("function.vk"), "declaration/function_debug.rkt").expect("function");
}

#[test]
fn lex_function() {}

#[test]
fn test_apply() {
    // let raw = "::(B)";
    // let apply = GenericArgument::parse_text(raw).unwrap();
    // pretty_print(&apply)
}

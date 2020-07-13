use super::*;

#[test]
fn lex_namespace() {
    top_debug(include_str!("namespaces.vk"), "declaration/namespaces_debug.rkt").expect("namespaces");
}

#[test]
fn lex_classes() {
    top_debug(include_str!("class.vk"), "declaration/class_debug.rkt").expect("class");
}

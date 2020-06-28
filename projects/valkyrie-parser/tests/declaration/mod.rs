use super::*;

#[test]
fn lex_namespace() {
    top_debug(include_str!("namespaces.vk"), "declaration/namespaces_debug.rkt").expect("namespaces");
}
//

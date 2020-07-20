use super::*;
use valkyrie_ast::GenericArgumentNode;

#[test]
fn lex_namespace() {
    top_debug(include_str!("namespaces.vk"), "declaration/namespaces_debug.rkt").expect("namespaces");
    top_debug(include_str!("class.vk"), "declaration/class_debug.rkt").expect("class");
    top_debug(include_str!("function.vk"), "declaration/function_debug.rkt").expect("function");
}

#[test]
fn lex_function() {}

#[test]
fn test_apply() {
    let raw = "::<B>";
    let apply = GenericArgumentNode::parse_text(raw).unwrap();
    println!("{}", colored_lisp(apply.as_lisp(), 144).unwrap());
}

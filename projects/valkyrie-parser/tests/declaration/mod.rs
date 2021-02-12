use super::*;

#[test]
fn mlex_declarations() {
    parse_program(include_str!("namespaces.vk"), "declaration/namespaces.ron").expect("namespaces");
    parse_program(include_str!("import.vk"), "declaration/import.ron").expect("import");
    // top_debug(include_str!("function.vk"), "declaration/function_debug.rkt").expect("function");
}

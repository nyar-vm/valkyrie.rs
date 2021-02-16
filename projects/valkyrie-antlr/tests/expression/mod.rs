use super::*;

#[test]
fn antlr_expression() {
    top_debug(include_str!("apply.vk"), "expression/apply_debug.rkt").expect("apply");
    top_debug(include_str!("infix.vk"), "expression/infix.ron").expect("infix");
    top_debug(include_str!("unary.vk"), "expression/unary.ron").expect("unary");
    // top_debug(include_str!("slice.vk"), "expression/slice_debug.rkt").expect("slice");
    // top_debug(include_str!("generic.vk"), "expression/generic_debug.rkt").expect("generic");
    // top_debug(include_str!("new.vk"), "expression/new_debug.rkt").expect("new");
}

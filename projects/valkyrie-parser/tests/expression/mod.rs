use super::*;
use valkyrie_ast::{ImportStatement, NamespaceDeclaration, NumberLiteralNode, PrettyPrint};

#[test]
fn lex_expression() {
    top_debug(include_str!("infix.vk"), "expression/infix_debug.rkt").expect("infix");
    top_debug(include_str!("unary.vk"), "expression/unary_debug.rkt").expect("unary");
    top_debug(include_str!("table.vk"), "expression/table_debug.rkt").expect("table");
    top_debug(include_str!("apply.vk"), "expression/apply_debug.rkt").expect("apply");
    top_debug(include_str!("slice.vk"), "expression/slice_debug.rkt").expect("slice");
    top_debug(include_str!("generic.vk"), "expression/generic_debug.rkt").expect("generic");
    top_debug(include_str!("new.vk"), "expression/new_debug.rkt").expect("new");
}

#[test]
fn test_apply2() {}

#[test]
fn test_apply() {
    let raw = "using `python`.numpy";
    let apply = ImportStatement::parse_text(raw).unwrap();
    apply.pretty_print(42)
}

#[test]
fn main2() {
    let raw = "⁅:, ::, : :, 1, :index0:, ::-1, i::j, i: :j⁆";
    let slice = SubscriptNode::parse_text(raw).unwrap();
    slice.pretty_print(42)
}

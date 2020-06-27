use super::*;
use valkyrie_ast::{GenericCall, TermExpressionNode, ViewNode};
use valkyrie_parser::ThisParser;

#[test]
fn lex_expression() {
    repl_debug(include_str!("infix.vk"), "expression/infix_debug.rkt").expect("infix");
    repl_debug(include_str!("unary.vk"), "expression/unary_debug.rkt").expect("unary");
    repl_debug(include_str!("table.vk"), "expression/table_debug.rkt").expect("table");
    repl_debug(include_str!("apply.vk"), "expression/apply_debug.rkt").expect("apply");
    repl_debug(include_str!("slice.vk"), "expression/slice_debug.rkt").expect("slice");
    repl_debug(include_str!("generic.vk"), "expression/generic_debug.rkt").expect("generic");
}

#[test]
fn test_apply2() {}

#[test]
fn test_apply() {
    let raw = "a.b";
    let apply = TermExpressionNode::parse_text(raw).unwrap();
    println!("{}", colored_lisp(apply.as_lisp(), 144).unwrap());
}

#[test]
fn main1() {
    let raw = "A:: <T, 1, >";
    let slice = GenericCall::<TermExpressionNode>::parse_text(raw).unwrap();
    println!("input:\n{slice:#?}");
    println!("output:\n{}", colored_lisp(slice.as_lisp(), 42).unwrap());
}

#[test]
fn main2() {
    let raw = "⁅:, ::, : :, 1, :index0:, ::-1, i::j, i: :j⁆";
    let slice = ViewNode::parse_text(raw).unwrap();
    println!("input:\nplaceholder{raw}");
    println!("output:\n{}", colored_lisp(slice.as_lisp(), 42).unwrap());
}

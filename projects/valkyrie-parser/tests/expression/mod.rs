use super::*;
use valkyrie_ast::{IdentifierNode, ViewNode};
use valkyrie_parser::ThisParser;

#[test]
fn lex_expression() {
    repl_debug(include_str!("infix.vk"), "expression/infix_debug.rkt").expect("infix");
    repl_debug(include_str!("unary.vk"), "expression/unary_debug.rkt").expect("unary");
    repl_debug(include_str!("table.vk"), "expression/table_debug.rkt").expect("table");
    // repl_debug(include_str!("tuple.vk"), "expression/tuple_debug.rkt").expect("tuple");
}

#[test]
fn test_expr() {
    let raw = r#"
.a(x, b: 1, c)
"#;
    let slice = ValkyrieDotCall::from_str(raw).unwrap();
    println!("input:\n{raw}");
    // println!("output:\n {call_index:#?}");
    println!("output:\n{}", colored_lisp(slice.lispify(), 42).unwrap());
}

#[test]
fn test_apply() {
    let raw = "ℤ";
    let apply = parse_repl(raw);
    for expr in &apply {
        println!("{}", colored_lisp(expr.lispify(), 144).unwrap());
    }
}

#[test]
fn main1() {
    let raw = "ℤ";
    let slice = IdentifierNode::parse_text(raw).unwrap();
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

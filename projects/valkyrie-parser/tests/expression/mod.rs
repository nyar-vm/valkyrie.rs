use super::*;

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
    let raw = "1 2 3";
    let apply = parse_repl(raw);
    for expr in &apply {
        println!("{}", colored_lisp(expr.lispify(), 144).unwrap());
    }
}

#[test]
fn main1() {
    let raw = "[1, :, ::, :n:, index::range, index: :range]";
    let slice = ViewNode::parse(ParseState::new(raw)).unwrap();
    println!("input:\nplaceholder{raw}");
    // println!("output:\n {expr:#?}");
    println!("output:\n{}", colored_lisp(slice.lispify(), 42).unwrap());
}

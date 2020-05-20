use super::*;

#[test]
fn lex_expression() {
    repl_debug(include_str!("infix.vk"), "expression/infix_debug.rkt").expect("infix");
    repl_debug(include_str!("unary.vk"), "expression/unary_debug.rkt").expect("unary");
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
fn test_table() {
    let raw = r#"[
    2,
    3,
    a: 1, 
    b: 2,
#   *args,
#   **kwargs
]"#;
    let slice = ValkyrieTable::from_str(raw).unwrap();
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
    let slice = ValkyrieView::parse(ParseState::new(raw)).unwrap();
    println!("input:\nplaceholder{raw}");
    // println!("output:\n {expr:#?}");
    println!("output:\n{}", colored_lisp(slice.lispify(), 42).unwrap());
}

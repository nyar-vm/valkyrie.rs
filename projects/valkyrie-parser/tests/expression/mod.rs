use lispify::{helpers::colored_lisp, Lispify};
use pex::ParseState;
use std::str::FromStr;

use valkyrie_parser::{
    call_apply::ValkyrieApply,
    call_dot::ValkyrieDotCall,
    call_index::ValkyrieView,
    expression::{ExpressionResolver, ExpressionStream},
    table::ValkyrieTable,
};

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
    let raw = "(1, 2, 3, a: 2, `b`: value)";
    let apply = ValkyrieApply::parse(ParseState::new(raw)).unwrap();
    println!("input:\nplaceholder{raw}");
    // println!("output:\n {call_apply:#?}");
    println!("output:\n{}", colored_lisp(apply.lispify(), 42).unwrap());
}

#[test]
fn main1() {
    let raw = "[1, :, ::, :n:, index::range, index: :range]";
    let slice = ValkyrieView::parse(ParseState::new(raw)).unwrap();
    println!("input:\nplaceholder{raw}");
    // println!("output:\n {expr:#?}");
    println!("output:\n{}", colored_lisp(slice.lispify(), 42).unwrap());
}

#[test]
fn main() {
    let resolver = ExpressionResolver::default();
    let raw = "5cm + 20℃ + global::x? ^ 3 ^ (4 - '2') + 5 * (y! - 1) * 7 ++ list";
    let tt = ExpressionStream::parse(ParseState::new(raw)).unwrap();
    let expr = resolver.resolve(tt).unwrap();
    println!("input:\n{raw}");
    // println!("output:\n {expr:#?}");
    println!("output:\n{}", colored_lisp(expr.lispify(), 42).unwrap());
}

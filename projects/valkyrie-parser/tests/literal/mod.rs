// use valkyrie_errors::ValkyrieResult;
//
// use crate::debug_lexer;
//
// #[test]
// fn test_atomic() -> ValkyrieResult {
//     debug_lexer(&["tests/literal/symbol.vk", "tests/literal/binary.vk"])
// }
//
// #[test]
// fn test_number() -> ValkyrieResult {
//     debug_lexer(&["tests/literal/number.vk"])
// }
//
// #[test]
// fn test_string() -> ValkyrieResult {
//     debug_lexer(&["tests/literal/string.vk", "tests/literal/escape.vk"])
// }
//
// #[test]
// fn test_composite() {
//     debug_lexer(&["tests/literal/tuple.vk", "tests/literal/table.vk"]).unwrap();
// }

use lispify::{helpers::colored_lisp, Lispify};
use std::str::FromStr;
use valkyrie_parser::{number::ValkyrieNumber, repl::parse_repl, string::ValkyrieString};

#[test]
fn test_apply() {
    let apply = parse_repl(include_str!("number.vk")).expect("failed to parse");
    for expr in &apply {
        println!("{}", colored_lisp(expr.lispify(), 144).unwrap());
    }
}

#[test]
fn test_number() {
    let n = ValkyrieNumber::from_str("1_cm").expect("1");
    println!("{}", colored_lisp(n.lispify(), 144).unwrap());
}

#[test]
fn test_string() {
    let n = ValkyrieString::from_str(r#"''''a\"""\''''"#).expect("1");
    println!("{}", colored_lisp(n.lispify(), 144).unwrap());
}

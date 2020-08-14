use super::*;
use valkyrie_ast::StringLiteralNode;

#[test]
fn lex_literal() {
    repl_debug(include_str!("number.vk"), "literal/number_debug.rkt").expect("number");
    repl_debug(include_str!("symbol.vk"), "literal/symbol_debug.rkt").expect("symbol");
    repl_debug(include_str!("string.vk"), "literal/string_debug.rkt").expect("string");
    repl_debug(include_str!("multiline.vk"), "literal/multiline_debug.rkt").expect("multiline");
    repl_debug(include_str!("tuple.vk"), "literal/table_debug.rkt").expect("table");
    repl_debug(include_str!("table.vk"), "literal/table_debug.rkt").expect("table");
}

#[test]
fn test_table() {}

#[test]
fn test_problems() {
    let raw = "t'a\n'";
    let apply = StringLiteralNode::parse_text(raw).unwrap();
    apply.pretty_print(42)
}

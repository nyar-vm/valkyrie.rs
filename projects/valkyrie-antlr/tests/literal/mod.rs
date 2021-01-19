use super::*;
// use valkyrie_ast::StringLiteralNode;

#[test]
fn antlr_literal() {
    top_debug(include_str!("escape.vk"), "literal/escape_debug.rkt").expect("escape");
    repl_debug(include_str!("multiline.vk"), "literal/multiline_debug.rkt").expect("multiline");
    repl_debug(include_str!("number.vk"), "literal/number_debug.rkt").expect("number");
    repl_debug(include_str!("symbol.vk"), "literal/symbol_debug.rkt").expect("symbol");
    repl_debug(include_str!("string.vk"), "literal/string_debug.rkt").expect("string");
    repl_debug(include_str!("table.vk"), "literal/table_debug.rkt").expect("table");
}

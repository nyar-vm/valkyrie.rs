use super::*;

#[test]
fn lex_number() {
    repl_debug(include_str!("number.vk"), "literal/number_debug.rkt").expect("number");
    repl_debug(include_str!("symbol.vk"), "literal/symbol_debug.rkt").expect("symbol");
    repl_debug(include_str!("multiline.vk"), "literal/multiline_debug.rkt").expect("multiline");
}

#[test]
fn test_table() {
    repl_debug(include_str!("table.vk"), "literal/table_debug.rkt").expect("table");
}

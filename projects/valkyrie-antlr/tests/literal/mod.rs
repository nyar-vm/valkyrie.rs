use super::*;
// use valkyrie_ast::StringLiteralNode;

#[test]
fn antlr_literal() {
    top_debug(include_str!("collection.vk"), "literal/collection_debug.rkt").expect("collection");
    top_debug(include_str!("escape.vk"), "literal/escape_debug.rkt").expect("escape");
    top_debug(include_str!("multiline.vk"), "literal/multiline_debug.rkt").expect("multiline");
    top_debug(include_str!("number.vk"), "literal/number_debug.rkt").expect("number");
    top_debug(include_str!("symbol.vk"), "literal/symbol_debug.rkt").expect("symbol");
    top_debug(include_str!("string.vk"), "literal/string_debug.rkt").expect("string");
}

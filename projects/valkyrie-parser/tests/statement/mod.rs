use super::*;
use valkyrie_ast::GenericArgumentNode;

#[test]
fn lex_looping() {
    top_debug(include_str!("import.vk"), "statement/import_debug.rkt").expect("import");
    repl_debug(include_str!("while.vk"), "statement/while_debug.rkt").expect("while");
}

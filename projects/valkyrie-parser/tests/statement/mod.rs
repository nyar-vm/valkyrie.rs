use super::*;

#[test]
fn lex_statements() {
    top_debug(include_str!("import.vk"), "statement/import_debug.rkt").expect("import");
    repl_debug(include_str!("while.vk"), "statement/while_debug.rkt").expect("while");
    top_debug(include_str!("lambda.vk"), "statement/lambda_debug.rkt").expect("lambda");
}

#[test]
fn debug_lex() {
    top_debug(include_str!("lambda.vk"), "statement/lambda_debug.rkt").expect("lambda");
}

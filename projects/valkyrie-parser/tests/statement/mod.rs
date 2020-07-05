use super::*;

#[test]
fn lex_looping() {
    repl_debug(include_str!("loop.vk"), "statement/loop_debug.rkt").expect("loop");
}

#[test]
fn lex_use() {
    top_debug(include_str!("import.vk"), "statement/import_debug.rkt").expect("import");
}

#[test]
fn test_apply() {
    let raw = "using a.b.c { c as abc }";
    let apply = StatementType::parse_text(raw).unwrap();
    println!("{}", colored_lisp(apply.as_lisp(), 144).unwrap());
}

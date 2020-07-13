use super::*;
use valkyrie_ast::ClassDeclarationNode;

#[test]
fn lex_looping() {
    repl_debug(include_str!("while.vk"), "statement/while_debug.rkt").expect("while");
}

#[test]
fn lex_use() {
    top_debug(include_str!("import.vk"), "statement/import_debug.rkt").expect("import");
}

#[test]
fn test_apply() {
    let raw = "class A { }";
    let apply = ClassDeclarationNode::parse_text(raw).unwrap();
    println!("{}", colored_lisp(apply.as_lisp(), 144).unwrap());
}

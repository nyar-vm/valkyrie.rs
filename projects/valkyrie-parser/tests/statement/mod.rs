use super::*;

#[test]
fn lex_statements() {
    top_debug(include_str!("import.vk"), "statement/import_debug.rkt").expect("import");
    top_debug(include_str!("define_flags.vk"), "statement/define_flags_debug.rkt").expect("flags");
    top_debug(include_str!("define_class.vk"), "statement/define_class_debug.rkt").expect("union");
    top_debug(include_str!("loop_for.vk"), "statement/loop_for_debug.rkt").expect("for");
    top_debug(include_str!("loop_while.vk"), "statement/loop_while_debug.rkt").expect("while");
    top_debug(include_str!("lambda.vk"), "statement/lambda_debug.rkt").expect("lambda");
    top_debug(include_str!("control.vk"), "statement/control_debug.rkt").expect("control");
    top_debug(include_str!("let_bind.vk"), "statement/let_bind_debug.rkt").expect("let");
    top_debug(include_str!("guard.vk"), "statement/guard_debug.rkt").expect("guard");
}

#[test]
fn debug_statement() {
    top_debug(include_str!("define_flags.vk"), "statement/define_flags_debug.rkt").expect("flags");
    top_debug(include_str!("define_class.vk"), "statement/define_class_debug.rkt").expect("union");
}

#[test]
fn test_statement() {
    let raw = ParseState::new("class A {a : Type = 1,}");
    let apply = ClassDeclaration::parse(raw).unwrap();
    apply.pretty_print(42)
}

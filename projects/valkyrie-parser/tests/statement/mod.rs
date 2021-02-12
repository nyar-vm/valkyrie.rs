use super::*;

#[test]
fn lex_statements() {
    top_debug(include_str!("../declaration/import.vk"), "statement/import_debug.rkt").expect("import");
    top_debug(include_str!("define_flags.vk"), "statement/define_flags_debug.rkt").expect("flags");
    top_debug(include_str!("define_class.vk"), "statement/define_class_debug.rkt").expect("class");
    top_debug(include_str!("loop_for.vk"), "statement/loop_for_debug.rkt").expect("for");
    top_debug(include_str!("loop_while.vk"), "statement/loop_while_debug.rkt").expect("while");
    top_debug(include_str!("lambda.vk"), "statement/lambda_debug.rkt").expect("lambda");
    top_debug(include_str!("control.vk"), "statement/control_debug.rkt").expect("control");
    top_debug(include_str!("jmp_if.vk"), "statement/jmp_if_debug.rkt").expect("if");
    top_debug(include_str!("jmp_guard.vk"), "statement/jmp_guard_debug.rkt").expect("guard");
    top_debug(include_str!("jmp_switch.vk"), "statement/jmp_switch_debug.rkt").expect("switch");
}

#[test]
fn debug_statement() {
    top_debug(include_str!("control.vk"), "statement/control_debug.rkt").expect("control");
}

#[test]
fn test_statement() {
    let raw = ParseState::new(
        r#"while 0 {

}"#,
    );
    let apply = WhileLoop::parse(raw).unwrap();
    pretty_print(&apply)
}

#[test]
fn test_statement2() {
    let raw = ParseState::new("switch {  }");
    let apply = SwitchStatement::parse(raw).unwrap();
    pretty_print(&apply)
}

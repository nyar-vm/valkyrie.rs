use super::*;

#[test]
fn antlr_statements() {
    // top_debug(include_str!("import.vk"), "statement/import_debug.rkt").expect("import");
    // top_debug(include_str!("define_flags.vk"), "statement/define_flags_debug.rkt").expect("flags");
    // top_debug(include_str!("define_class.vk"), "statement/define_class_debug.rkt").expect("class");
    top_debug(include_str!("if_else.vk"), "statement/if_else_debug.rkt").expect("if");
    top_debug(include_str!("if_let.vk"), "statement/if_let_debug.rkt").expect("guard");
    top_debug(include_str!("for_loop.vk"), "statement/for_loop_debug.rkt").expect("for");
    top_debug(include_str!("while_loop.vk"), "statement/while_loop_debug.rkt").expect("while");
    // top_debug(include_str!("lambda.vk"), "statement/lambda_debug.rkt").expect("lambda");
    // top_debug(include_str!("control.vk"), "statement/control_debug.rkt").expect("control");
    // top_debug(include_str!("jmp_switch.vk"), "statement/jmp_switch_debug.rkt").expect("switch");
}

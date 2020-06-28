use super::*;

#[test]
fn lex_looping() {
    repl_debug(include_str!("loop.vk"), "statement/loop_debug.rkt").expect("loop");
}
//

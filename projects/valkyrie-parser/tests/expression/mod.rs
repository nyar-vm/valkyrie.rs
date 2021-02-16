use super::*;
use valkyrie_parser::{MainExpressionNode, MainStatementNode, ValkyrieRule::MainStatement};

#[test]
fn lex_expression() {
    parse_expression(include_str!("unary.vk"), "expression/unary.ron").expect("unary");
    parse_expression(include_str!("infix.vk"), "expression/infix.ron").expect("infix");

    // top_debug(include_str!("table.vk"), "expression/table_debug.rkt").expect("table");
    // top_debug(include_str!("apply.vk"), "expression/apply_debug.rkt").expect("apply");
    // top_debug(include_str!("slice.vk"), "expression/slice_debug.rkt").expect("slice");
    // top_debug(include_str!("generic.vk"), "expression/generic_debug.rkt").expect("generic");
    // top_debug(include_str!("new.vk"), "expression/new_debug.rkt").expect("new");
}

fn debug_expression(input: &str) -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let cst = ValkyrieParser::parse_cst(input, ValkyrieRule::MainStatement).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = MainStatementNode::from_str(input).unwrap();
    println!("Long Form:\n{:#?}", ast);
    Ok(())
}

#[test]
fn test_apply() {
    let raw = "1 in +2";
    debug_expression(raw).unwrap();
}

// #[test]
// fn main2() {
//     let raw = "⁅:, ::, : :, 1, :index0:, ::-1, i::j, i: :j⁆";
//     let slice = SubscriptCallNode::parse_text(raw).unwrap();
//     pretty_print(&slice)
// }

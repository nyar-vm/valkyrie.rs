use super::*;
use valkyrie_parser::MainExpressionNode;

#[test]
fn lex_expression() {
    // top_debug(include_str!("infix.vk"), "expression/infix_debug.rkt").expect("infix");
    // top_debug(include_str!("unary.vk"), "expression/unary_debug.rkt").expect("unary");
    // top_debug(include_str!("table.vk"), "expression/table_debug.rkt").expect("table");
    // top_debug(include_str!("apply.vk"), "expression/apply_debug.rkt").expect("apply");
    // top_debug(include_str!("slice.vk"), "expression/slice_debug.rkt").expect("slice");
    // top_debug(include_str!("generic.vk"), "expression/generic_debug.rkt").expect("generic");
    // top_debug(include_str!("new.vk"), "expression/new_debug.rkt").expect("new");
}

fn parse_expression(input: &str) -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let cst = ValkyrieParser::parse_cst(input, ValkyrieRule::MainExpression).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = MainExpressionNode::from_str(input).unwrap();
    println!("Long Form:\n{:#?}", ast);
    Ok(())
}

#[test]
fn test_apply() {
    let raw = "1 + +2";
    parse_expression(raw).unwrap();
}

// #[test]
// fn main2() {
//     let raw = "⁅:, ::, : :, 1, :index0:, ::-1, i::j, i: :j⁆";
//     let slice = SubscriptCallNode::parse_text(raw).unwrap();
//     pretty_print(&slice)
// }

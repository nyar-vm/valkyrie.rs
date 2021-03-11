use nyar_error::third_party::Url;
use std::path::PathBuf;
use valkyrie_parser::{MainStatementNode, ProgramContext, RangeLiteralNode};

use super::*;

fn take_expression(input: StatementNode) -> Option<MainExpressionNode> {
    todo!()
}

#[test]
fn lex_expression() {
    // top_debug(include_str!("table.vk"), "expression/table_debug.rkt").expect("table");
    // top_debug(include_str!("apply.vk"), "expression/apply_debug.rkt").expect("apply");
    // top_debug(include_str!("generic.vk"), "expression/generic_debug.rkt").expect("generic");
    // top_debug(include_str!("new.vk"), "expression/new_debug.rkt").expect("new");
}

fn debug_expression(input: &str) -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let cst = ValkyrieParser::parse_cst(input, ValkyrieRule::MainStatement).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = MainExpressionNode::from_str(input).unwrap();
    println!("Long Form:\n{:#?}", ast);
    Ok(())
}

#[test]
fn debug() {
    let raw = "[1:1]";
    debug_expression(raw).unwrap();
}

// #[test]
// fn main2() {
//     let raw = "⁅:, ::, : :, 1, :index0:, ::-1, i::j, i: :j⁆";
//     let slice = SubscriptCallNode::parse_text(raw).unwrap();
//     pretty_print(&slice)
// }

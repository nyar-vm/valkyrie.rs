use nyar_error::{third_party::Url, FileID};
use std::path::PathBuf;
use valkyrie_ast::ProgramRoot;
use valkyrie_parser::{MainStatementNode, ProgramContext, RangeLiteralNode};

use super::*;

fn debug_literal(input: &str) -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let cst = ValkyrieParser::parse_cst(input, ValkyrieRule::Program).unwrap();
    println!("Short Form:\n{}", cst);
    let context = ProgramContext { file: unsafe { FileID::new(0) } };
    let ast = ProgramNode::from_str(input).unwrap().build(&context).unwrap();
    println!("Long Form:\n{:#?}", ast);
    Ok(())
}

#[test]
fn debug() {
    let raw = r#"
();
(0);
(1, );
(1, 2, );
(true, (true, ), ((true, (()))));
    "#;
    debug_literal(raw).unwrap();
}

// #[test]
// fn main2() {
//     let raw = "⁅:, ::, : :, 1, :index0:, ::-1, i::j, i: :j⁆";
//     let slice = SubscriptCallNode::parse_text(raw).unwrap();
//     pretty_print(&slice)
// }

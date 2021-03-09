use nyar_error::{third_party::Url, FileID};
use std::path::PathBuf;
use valkyrie_ast::ProgramRoot;
use valkyrie_parser::{MainStatementNode, ProgramContext, RangeLiteralNode};

use super::*;

fn parse_literal(file: &str) -> anyhow::Result<Vec<MainExpressionNode>> {
    let (input, output, path) = read_io("literal", file)?;
    let cst = ValkyrieParser::parse_cst(&input, ValkyrieRule::Program).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = match ProgramNode::from_str(&input) {
        Ok(s) => s.statement.into_iter().flat_map(|v| take_expression(v)).collect(),
        Err(_) => {
            vec![]
        }
    };
    let mut file = File::create(path)?;
    let new = format!("{:#?}", ast);
    file.write_all(new.as_bytes());
    assert_eq!(new, output);
    Ok(ast)
}

fn take_expression(input: StatementNode) -> Option<MainExpressionNode> {
    match input {
        StatementNode::MainStatement(MainStatementNode::MainExpression(e)) => Some(e),
        _ => None,
    }
}

#[test]
fn test_number() {
    parse_literal("number").unwrap();
}

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

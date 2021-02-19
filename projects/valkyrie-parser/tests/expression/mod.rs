use nyar_error::third_party::Url;
use std::path::PathBuf;
use valkyrie_parser::MainStatementNode;

use super::*;

fn parse_expression(file: &str) -> anyhow::Result<Vec<MainExpressionNode>> {
    let (input, output, path) = read_io("expression", file)?;
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
fn test_unary() {
    parse_expression("unary").unwrap();
}

#[test]
fn test_infix() {
    parse_expression("infix").unwrap();
}

#[test]
fn test_slice() {
    parse_expression("slice").unwrap();
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
    let ast = MainStatementNode::from_str(input).unwrap();
    println!("Long Form:\n{:#?}", ast);
    Ok(())
}

#[test]
fn test_apply() {
    let raw = "a == c";
    debug_expression(raw).unwrap();
}

// #[test]
// fn main2() {
//     let raw = "⁅:, ::, : :, 1, :index0:, ::-1, i::j, i: :j⁆";
//     let slice = SubscriptCallNode::parse_text(raw).unwrap();
//     pretty_print(&slice)
// }

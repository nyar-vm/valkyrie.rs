#![allow(unused, dead_code)]

use std::{fs::File, io::Write, path::Path, str::FromStr};

use yggdrasil_rt::{YggdrasilError, YggdrasilParser};

use valkyrie_parser::{
    MainExpressionNode, MainStatementNode, ProgramNode, StatementNode, ValkyrieParser, ValkyrieRule,
    ValkyrieRule::MainStatement,
};

mod declaration;
mod expression;
// mod expression;
// mod literal;
// mod statement;
//
// use lispify::PrettyPrint as _;
// use pex::ParseState;
// use std::{
//     fs::File,
//     io::Write,
//     path::{Path, PathBuf},
// };
// use valkyrie_ast::{
//     helper::{PrettyPrint, PrettyProvider},
//     *,
// };
// use valkyrie_parser::{ReplRoot, ThisParser};

#[test]
fn ready() {
    println!("it works!")
}
//
// fn here() -> PathBuf {
//     Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize().expect("failed to get manifest dir")
// }

// pub fn pretty_print<T: PrettyPrint>(value: &T) {
//     let arena = PrettyProvider::new(80);
//     println!("{}", value.pretty_colorful(&arena));
// }

fn parse_program(input: &str, output: &str) -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let cst = ValkyrieParser::parse_cst(input, ValkyrieRule::Program).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = ProgramNode::from_str(input).unwrap();
    let mut file = File::create(here.join(output))?;
    file.write_all(format!("{:#?}", ast).as_bytes())
}

fn parse_expression(input: &str, output: &str) -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let cst = ValkyrieParser::parse_cst(input, ValkyrieRule::Program).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = match ProgramNode::from_str(input) {
        Ok(s) => s.statement.into_iter().flat_map(|v| take_expression(v)).collect(),
        Err(_) => {
            vec![]
        }
    };
    let mut file = File::create(here.join(output))?;
    file.write_all(format!("{:#?}", ast).as_bytes())
}

fn take_expression(input: StatementNode) -> Option<MainExpressionNode> {
    match input {
        StatementNode::MainStatement(MainStatementNode::MainExpression(e)) => Some(e),
        _ => None,
    }
}

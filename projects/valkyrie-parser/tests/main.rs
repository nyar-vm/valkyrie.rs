#![allow(unused, dead_code)]

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

use nyar_error::third_party::Url;
use valkyrie_parser::{
    MainExpressionNode, MainStatementNode, ProgramNode, StatementNode, ValkyrieParser, ValkyrieRule,
    ValkyrieRule::MainStatement,
};
use yggdrasil_rt::{YggdrasilError, YggdrasilParser};

mod declaration;
mod expression;

mod literal;
// mod statement;

#[test]
fn ready() {
    println!("it works!")
}
//
fn here() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize().expect("failed to get manifest dir")
}

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

fn read_io(dir: &str, file: &str) -> std::io::Result<(String, String, PathBuf)> {
    let here = here();
    let input = here.join(dir).join(format!("{}.vk", file)).canonicalize()?;
    let output = here.join(dir).join(format!("{}.ron", file)).canonicalize()?;
    if let Ok(o) = Url::from_file_path(&input) {
        println!("Parsing: {}", o)
    }
    let in_text = std::fs::read_to_string(&input)?;
    let out_text = std::fs::read_to_string(&output)?;
    Ok((in_text, out_text, output))
}

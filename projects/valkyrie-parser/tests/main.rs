mod declaration;
mod expression;
mod literal;
mod statement;

use lispify::{
    helpers::{colored_lisp, display_lisp},
    Lispify,
};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use valkyrie_ast::{StatementNode, StatementType};
use valkyrie_parser::ThisParser;

#[test]
fn ready() {
    println!("it works!")
}

fn here() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize().expect("failed to get manifest dir")
}

fn top_debug(text: &str, output: &str) -> std::io::Result<()> {
    let mut file = File::create(here().join(output))?;
    let apply = StatementType::parse_many(text).unwrap();
    for expr in &apply {
        println!("{}", expr);
        writeln!(file, "{}", display_lisp(expr.as_lisp(), 144).unwrap())?;
        println!("{}", colored_lisp(expr.as_lisp(), 144).unwrap());
    }
    Ok(())
}

fn repl_debug(text: &str, output: &str) -> std::io::Result<()> {
    let mut file = File::create(here().join(output))?;
    let apply = StatementNode::parse_many(text).unwrap();
    for expr in &apply {
        println!("{}", expr);
        writeln!(file, "{}", display_lisp(expr.as_lisp(), 144).unwrap())?;
        println!("{}", colored_lisp(expr.as_lisp(), 144).unwrap());
    }
    Ok(())
}

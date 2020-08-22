#![allow(unused, dead_code)]
mod declaration;
mod expression;
mod literal;
mod statement;

use lispify::helpers::{colored_lisp, display_lisp};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use valkyrie_ast::{ExpressionBody, PrettyPrint, StatementBody, StatementNode, SubscriptNode};
use valkyrie_parser::{ReplRoot, ScriptRoot, ThisParser};

#[test]
fn ready() {
    println!("it works!")
}

fn here() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize().expect("failed to get manifest dir")
}

fn top_debug(text: &str, output: &str) -> std::io::Result<()> {
    let mut file = File::create(here().join(output))?;
    let apply = ScriptRoot::parse_text(text).unwrap();
    for expr in &apply.statements {
        expr.pretty_print(80);
        writeln!(file, "{}", display_lisp(expr.as_lisp(), 144).unwrap())?;
    }
    Ok(())
}

fn repl_debug(text: &str, output: &str) -> std::io::Result<()> {
    let mut file = File::create(here().join(output))?;
    let apply = ReplRoot::parse_text(text).unwrap();
    for expr in &apply.statements {
        expr.pretty_print(80);
        writeln!(file, "{}", display_lisp(expr.as_lisp(), 144).unwrap())?;
        println!("{}", colored_lisp(expr.as_lisp(), 144).unwrap());
    }
    Ok(())
}

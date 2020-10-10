#![allow(unused, dead_code)]
mod declaration;
mod expression;
mod literal;
mod statement;

use lispify::PrettyPrint as _;
use pex::ParseState;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use valkyrie_ast::*;
use valkyrie_parser::{ReplRoot, ThisParser};

#[test]
fn ready() {
    println!("it works!")
}

fn here() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize().expect("failed to get manifest dir")
}

pub fn pretty_print<T: PrettyPrint>(value: &T) {
    let arena = PrettyProvider::new(80);
    println!("{}", value.pretty_colorful(&arena));
}

fn top_debug(text: &str, output: &str) -> std::io::Result<()> {
    let mut file = File::create(here().join(output))?;
    let apply = ProgramRoot::parse_text(text).unwrap();
    let mut theme = PrettyProvider::new(128);
    for expr in &apply.statements {
        println!("=================================================================");
        pretty_print(expr);
        let text = expr.pretty_colorful(&theme);
        let lisp = expr.as_lisp();
        writeln!(file, "{}", lisp.pretty_string(&theme))?;
        println!("{}", lisp.pretty_colorful(&theme));
    }
    Ok(())
}

fn repl_debug(text: &str, output: &str) -> std::io::Result<()> {
    let mut file = File::create(here().join(output))?;
    let apply = ReplRoot::parse_text(text).unwrap();
    let mut theme = PrettyProvider::new(128);
    for expr in &apply.statements {
        println!("=================================================================");
        pretty_print(expr);
        let text = expr.pretty_colorful(&theme);
        let lisp = expr.as_lisp();
        writeln!(file, "{}", lisp.pretty_string(&theme))?;
        println!("{}", lisp.pretty_colorful(&theme));
    }
    Ok(())
}

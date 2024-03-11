use std::{fmt::Arguments, fs::File, path::Path};
use valkyrie_types::ResolveContext;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_hello_world() -> nyar_error::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut context = ResolveContext::new("std");
    context.resolve_package(here.join("tests/source"))?;
    context.show_errors();
    let mut wat = File::create(here.join("tests/component.wat"))?;
    let source = context.resolve()?;
    let wast = source.encode();
    wat.write_all(wast.as_bytes())?;
    Ok(())
}

struct X {}

impl std::io::Write for X {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }
    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        todo!()
    }
}

impl std::fmt::Write for X {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        todo!()
    }
}

use nyar_error::SourceCache;
use std::{fs::File, io::Write, path::Path};
use valkyrie_types::ResolveContext;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_hello_world() -> nyar_error::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mut context = ResolveContext::new("std");
    context.resolve_file(here.join("tests/source/unstable.vk"))?;
    context.resolve_file(here.join("tests/source/random/_.vk"))?;

    context.show_errors();

    let mut wat = File::create(here.join("tests/component.vk"))?;
    let source = context.resolve()?;
    let wast = source.encode();
    wat.write_all(wast.as_bytes())?;
    Ok(())
}

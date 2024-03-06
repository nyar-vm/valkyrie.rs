use nyar_error::FileCache;
use std::{path::Path, process::Command};
use std::io::Write;
use nyar_wasm::CanonicalWasi;
use valkyrie_types::define_io_types;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test_hello_world() {
    let mut context= Resolv
    
    let source = include_str!("component.vk");
    
    let component = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/component.wat");
    let mut wat = std::fs::File::create(component).unwrap();

    let source = CanonicalWasi::new(define_io_types()).unwrap();

    let wast = source.encode();
    wat.write_all(wast.as_bytes()).unwrap();
}

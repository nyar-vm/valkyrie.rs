use nyar_error::FileCache;
use std::path::Path;
use valkyrie_types::ModuleResolver;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/test.vk").canonicalize().unwrap();
    let mut cache = FileCache::default();
    let file = cache.load_local(file).unwrap();
    let mut reolver = ModuleResolver::default();
    for error in reolver.parse(file, &mut cache) {
        error.as_report().eprint(&cache).unwrap()
    }
    println!("{:#?}", reolver)
}

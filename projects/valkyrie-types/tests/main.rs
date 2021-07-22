use nyar_error::FileCache;
use std::path::Path;
use valkyrie_parser::ProgramContext;
use valkyrie_types::{testing::assert_type, ModuleResolver, ValkyrieID, ValkyrieInterface, ValkyrieList};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_list_index() {
    let out = ValkyrieList::from_iter(vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']);
    println!("1: {:?}", out.get_ordinal(1));
    println!("2: {:?}", out.get_ordinal(-1));
    println!("2: {:?}", out.get_range(1, -1, 1).collect::<Vec<_>>());
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

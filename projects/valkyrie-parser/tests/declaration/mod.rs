use super::*;
use std::{fs::File, io::Write, str::FromStr};
use valkyrie_parser::{ProgramNode, ValkyrieParser, ValkyrieRule};
use yggdrasil_rt::YggdrasilParser;

// #[test]
// fn mlex_declarations() {
//     top_debug(include_str!("namespaces.vk"), "declaration/namespaces.ron").expect("namespaces");
//     top_debug(include_str!("class.vk"), "declaration/class_debug.rkt").expect("class");
//     // top_debug(include_str!("function.vk"), "declaration/function_debug.rkt").expect("function");
// }

#[test]
fn test_ns() {
    let cst = ValkyrieParser::parse_cst(include_str!("namespaces.vk"), ValkyrieRule::Program).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = ProgramNode::from_str(include_str!("namespaces.vk")).unwrap();
    let mut file = File::create("tests/declaration/namespaces.ron").unwrap();
    file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}

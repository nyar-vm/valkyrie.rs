use super::*;
use valkyrie_parser::{DefineImportNode, DefineNamespaceNode};

#[test]
fn debug_declarations() {
    parse_program(include_str!("namespaces.vk"), "declaration/namespaces.ron").expect("namespaces");
    parse_program(include_str!("import.vk"), "declaration/import.ron").expect("import");
    // top_debug(include_str!("function.vk"), "declaration/function_debug.rkt").expect("function");
}

#[test]
fn debug_import() {
    parse_import("using;").expect("namespaces");
}

fn parse_import(input: &str) -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let cst = ValkyrieParser::parse_cst(input, ValkyrieRule::DefineImport).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = DefineImportNode::from_str(input).unwrap();
    println!("Long Form:\n{:#?}", ast);
    Ok(())
}

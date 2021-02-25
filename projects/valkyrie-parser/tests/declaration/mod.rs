use super::*;
use valkyrie_parser::{
    ClassBlockItemNode, ClassBlockNode, ClassFieldNode, ClassMethodNode, DefineImportNode, DefineNamespaceNode,
};

fn parse_program(file: &str) -> anyhow::Result<ProgramNode> {
    let (input, output, path) = read_io("declaration", file)?;
    let cst = ValkyrieParser::parse_cst(&input, ValkyrieRule::Program).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = ProgramNode::from_str(&input).unwrap();
    let mut file = File::create(path)?;
    let new = format!("{:#?}", ast);
    file.write_all(new.as_bytes());
    assert_eq!(new, output);
    Ok(ast)
}

#[test]
fn test_namespace() {
    parse_program("namespaces").unwrap();
}

#[test]
fn test_import() {
    parse_program("import").unwrap();
}
#[test]
fn test_class() {
    parse_program("class").unwrap();
}

#[test]
fn debug_import() {
    parse_import("{id field; yes method() }").expect("class method");
}

fn parse_import(input: &str) -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let cst = ValkyrieParser::parse_cst(input, ValkyrieRule::ClassBlock).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = ClassBlockNode::from_str(input).unwrap();
    println!("Long Form:\n{:#?}", ast);
    Ok(())
}

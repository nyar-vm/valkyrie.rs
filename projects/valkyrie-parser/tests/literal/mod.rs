use nyar_error::{third_party::Url, FileID};
use std::path::PathBuf;
use valkyrie_ast::ProgramRoot;
use valkyrie_parser::{MainStatementNode, ProgramContext, RangeLiteralNode};

use super::*;

fn debug_literal(input: &str) -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let cst = ValkyrieParser::parse_cst(input, ValkyrieRule::Program).unwrap();
    println!("Short Form:\n{}", cst);
    let mut cache = FileCache::default();
    let file = cache.load_text(input, "debug.v");
    let parsed = ProgramContext { file }.parse(&mut cache);
    match parsed {
        Success { value, diagnostics } => {
            println!("Long Form:\n{:#?}", value);
            for error in diagnostics {
                error.as_report().eprint(&cache)?;
            }
        }
        Failure { fatal, diagnostics } => {
            fatal.as_report().eprint(&cache)?;
            for error in diagnostics {
                error.as_report().eprint(&cache)?;
            }
        }
    }
    Ok(())
}

#[test]
fn debug() {
    let raw = r#"
();
(0);
(1, );
(1, 2, );
(true, (true, ), ((true, (()))));
    "#;
    debug_literal(raw).unwrap();
}

// #[test]
// fn main2() {
//     let raw = "⁅:, ::, : :, 1, :index0:, ::-1, i::j, i: :j⁆";
//     let slice = SubscriptCallNode::parse_text(raw).unwrap();
//     pretty_print(&slice)
// }

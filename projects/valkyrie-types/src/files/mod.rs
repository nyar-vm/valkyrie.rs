// use nyar_error::third_party::Url;
// use std::path::PathBuf;
// use valkyrie_ast::ProgramRoot;
//
// #[salsa_2022::input]
// pub struct ValkyrieFile {
//     pub path: ValkyrieFileLocation,
//     #[return_ref]
//     pub contents: String,
// }
//
// pub enum ValkyrieFileLocation {
//     Snippet(String),
//     Local(PathBuf),
//     Remote(Url),
// }
//
// #[salsa_2022::tracked]
// fn parse_file(db: &dyn crate::Db, file: ValkyrieFile) -> ProgramRoot {
//     let contents: &str = file.contents(db);
// }
//
// #[salsa::tracked]
// struct Ast {
//     #[return_ref]
//     top_level_items: Vec<Item>,
// }
//
// #[salsa::tracked]
// struct Item {
//     #[id]
//     name: Word, // 稍后会定义 Word
//     ...
// }

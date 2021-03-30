use crate::ProgramNode;
use nyar_error::{Failure, FileCache, FileID, NyarError, Validation};
use std::str::FromStr;
use valkyrie_ast::ProgramRoot;

pub struct ProgramContext {
    pub file: FileID,
}

impl ProgramContext {
    pub fn parse(&self, cache: &mut FileCache) -> Validation<ProgramRoot> {
        let text = cache.fetch(&self.file)?.to_string();
        match ProgramNode::from_str(&text) {
            Ok(o) => o.build(&self),
            Err(e) => Failure { fatal: NyarError::from(e).with_file(self.file), diagnostics: vec![] },
        }
    }
}

use crate::ProgramNode;
use nyar_error::{Failure, FileCache, FileID, NyarError, Validation};
use std::{ops::Range, str::FromStr};
use valkyrie_ast::ProgramRoot;

pub struct ProgramContext {
    pub file: FileID,
}

pub(crate) struct ProgramState {
    pub file: FileID,
    pub looper: LooperState,
}
pub(crate) struct LooperState {
    pub id: usize,
    pub stack: Vec<usize>,
}

impl ProgramState {
    pub fn new(file: FileID) -> Self {
        Self { file, looper: LooperState { id: 1, stack: vec![] } }
    }
    pub fn enter_looper(&mut self) -> String {
        self.looper.stack.push(self.looper.id);
        format!("looper-{}", self.looper.id)
    }
    pub fn exit_looper(&mut self) {
        self.looper.id += 1;
        self.looper.id = self.looper.stack.pop().expect("looper stack is empty");
    }
    pub fn reset_looper(&mut self) {
        self.looper.id = 1;
        self.looper.stack.clear()
    }
}

impl ProgramContext {
    pub fn parse(&self, cache: &mut FileCache) -> Validation<ProgramRoot> {
        let text = cache.fetch(&self.file)?.to_string();
        match ProgramNode::from_str(&text) {
            Ok(o) => o.build(&mut ProgramState::new(self.file)),
            Err(e) => Failure { fatal: NyarError::from(e).with_file(self.file), diagnostics: vec![] },
        }
    }
}

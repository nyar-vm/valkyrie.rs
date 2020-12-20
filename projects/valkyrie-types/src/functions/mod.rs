pub mod operators;

use crate::{types::ValkyrieMetaType, ValkyrieValue};
use std::{
    collections::BTreeMap,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};

pub trait ValkyrieFunctionType {
    fn boxed(self) -> ValkyrieValue;
    fn type_info(&self) -> Arc<ValkyrieMetaType>;
}

/// A function that can be called from Valkyrie.
///
/// ```vk
/// fun f(self, a, <, b, c, >, @a mut **a): Return {
///
/// }
/// ```
#[derive(Clone, Debug)]
pub struct ValkyrieFunction {
    pub name: String,
    pub span: DefinitionSpan,
    pub overloadable: bool,
    pub overridable: bool,
    pub arguments: Vec<ValkyrieValue>,
    pub return_type: Arc<ValkyrieMetaType>,
}
/// Actual function definition, non-overloading, non-overriding
pub struct ValkyrieMonomorphicFunction {
    pub require_continuation: bool,
    pub return_type: Arc<ValkyrieMetaType>,
}
/// Actual function definition, overloading, non-overriding
pub struct ValkyriePartialFunction {
    pub prototype: ValkyrieFunction,
    pub generic: Arc<ValkyrieMetaType>,
    pub arguments: Vec<ValkyrieValue>,
    pub named: BTreeMap<String, ValkyrieValue>,
}

/// if the code contains loop, it will be compiled into a state machine
pub struct ValkyrieFSM {}

#[derive(Clone, Debug)]
pub struct DefinitionSpan {
    file: String,
    range: Range<u32>,
}

impl DefinitionSpan {
    pub fn get_path<P>(&self, workspace: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let workspace = workspace.as_ref();
        let file = workspace.join(&self.file);
        file
    }
}

impl std::io::Write for DefinitionSpan {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

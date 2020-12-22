pub mod operators;

use crate::{types::ValkyrieMetaType, ValkyrieValue};
use std::{
    collections::BTreeMap,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use valkyrie_ast::StatementBlock;
use valkyrie_error::DefinitionSpan;

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
///
///
/// ```stack
/// 0: ret pointer
/// 1: env pointer
/// ```
#[derive(Clone, Debug)]
pub struct ValkyrieFunction {
    pub name: String,
    pub span: DefinitionSpan,
    pub overloadable: bool,
    pub overridable: bool,
    pub return_type: Arc<ValkyrieMonomorphicFunction>,
}

/// Actual function definition, non-overloading, non-overriding
#[derive(Clone, Debug)]
pub struct ValkyrieMonomorphicFunction {
    pub require_continuation: bool,
    pub arguments: Vec<ValkyrieValue>,
    pub return_type: Arc<ValkyrieMetaType>,
}

/// Actual function definition, overloading, non-overriding
#[derive(Clone, Debug)]
pub struct ValkyriePartialFunction {
    pub prototype: ValkyrieFunction,
    pub generic: Arc<ValkyrieMetaType>,
    pub arguments: Vec<ValkyrieValue>,
    pub named: BTreeMap<String, ValkyrieValue>,
    pub continuation: Option<StatementBlock>,
}

/// if the code contains loop, it will be compiled into a state machine
#[derive(Clone, Debug)]
pub struct ValkyrieFunctionInstance {
    pub return_type: ValkyrieValue,
}

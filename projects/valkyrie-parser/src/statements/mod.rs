use crate::{
    helpers::{ProgramContext, ProgramState},
    MainStatementNode, OpNamespaceNode,
};
use nyar_error::{Success, Validation};
use valkyrie_ast::*;
mod import;
mod namespace;

impl crate::ProgramNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ProgramRoot> {
        let mut errors = vec![];
        let mut terms = vec![];
        for node in &self.statement {
            node.build(ctx).append(&mut terms, &mut errors)
        }
        let statements = terms.into_iter().filter(|s| !matches!(s, StatementNode::Nothing)).collect();
        Success { value: ProgramRoot { statements }, diagnostics: errors }
    }
}

impl crate::StatementNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<StatementNode> {
        let value = match self {
            Self::DefineNamespace(v) => v.build(ctx).into(),
            Self::DefineClass(v) => v.build(ctx)?.into(),
            Self::DefineEnumerate(v) => v.build(ctx)?.into(),
            Self::DefineFunction(v) => v.build(ctx)?.into(),
            Self::DefineVariable(v) => v.build(ctx)?.into(),
            Self::DefineTrait(v) => v.build(ctx)?.into(),
            Self::DefineExtends(v) => v.build(ctx)?.into(),
            Self::DefineUnion(v) => v.build(ctx)?.into(),
            Self::MainStatement(v) => v.build(ctx)?,
        };
        Success { value, diagnostics: vec![] }
    }
}

impl crate::MainStatementNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<StatementNode> {
        let value = match self {
            Self::ControlFlow(v) => v.build(ctx)?.into(),
            Self::DefineImport(v) => v.build(ctx)?.into(),
            Self::ForStatement(v) => v.build(ctx)?.into(),
            Self::WhileStatement(v) => v.build(ctx)?.into(),
            Self::ExpressionRoot(v) => v.build(ctx)?.into(),
            Self::Eos(_) => StatementNode::Nothing,
        };
        Success { value, diagnostics: vec![] }
    }
}

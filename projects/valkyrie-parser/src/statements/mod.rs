use crate::{
    helpers::{ProgramContext, ProgramState},
    MainStatementNode, OpNamespaceNode,
};
use nyar_error::{Result, Success, Validation};
use valkyrie_ast::*;
mod import;
mod namespace;

impl crate::ProgramNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<ProgramRoot> {
        let mut statements = vec![];
        for node in &self.statement {
            node.build(ctx, &mut statements)?
        }
        Ok(ProgramRoot { statements })
    }
}

impl crate::StatementNode {
    pub fn build(&self, ctx: &mut ProgramState, ts: &mut Vec<StatementNode>) -> Result<()> {
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
        ts.push(value);
        Ok(())
    }
}

impl crate::MainStatementNode {
    pub fn build(&self, ctx: &mut ProgramState, ts: &mut Vec<StatementNode>) -> Result<()> {
        let value = match self {
            Self::ControlFlow(v) => v.build(ctx)?.into(),
            Self::DefineImport(v) => v.build(ctx)?.into(),
            Self::ForStatement(v) => v.build(ctx)?.into(),
            Self::WhileStatement(v) => v.build(ctx)?.into(),
            Self::ExpressionRoot(v) => v.build(ctx)?.into(),
            Self::Eos(_) => {}
        };
        Ok(())
    }
}

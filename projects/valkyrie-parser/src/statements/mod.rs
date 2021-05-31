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
        match self {
            Self::DefineNamespace(v) => ts.push(v.build(ctx).into()),
            Self::DefineClass(v) => ts.push(v.build(ctx)?.into()),
            Self::DefineEnumerate(v) => ts.push(v.build(ctx)?.into()),
            Self::DefineFunction(v) => ts.push(v.build(ctx)?.into()),
            Self::DefineVariable(v) => ts.push(v.build(ctx)?.into()),
            Self::DefineTrait(v) => ts.push(v.build(ctx)?.into()),
            Self::DefineExtends(v) => ts.push(v.build(ctx)?.into()),
            Self::DefineUnion(v) => ts.push(v.build(ctx)?.into()),
            Self::MainStatement(v) => v.build(ctx, ts)?,
        };
        Ok(())
    }
}

impl crate::MainStatementNode {
    pub fn build(&self, ctx: &mut ProgramState, ts: &mut Vec<StatementNode>) -> Result<()> {
        match self {
            Self::ControlFlow(v) => ts.push(v.build(ctx)?.into()),
            Self::DefineImport(v) => ts.push(v.build(ctx)?.into()),
            Self::ForStatement(v) => ts.push(v.build(ctx)?.into()),
            Self::WhileStatement(v) => ts.push(v.build(ctx)?.into()),
            Self::ExpressionRoot(v) => ts.push(v.build(ctx)?.into()),
            Self::Eos(_) => {}
        };
        Ok(())
    }
}

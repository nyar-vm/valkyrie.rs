use crate::{helpers::ProgramContext, OpNamespaceNode};
use nyar_error::{Success, Validation};
use valkyrie_ast::{ImportStatement, NamespaceDeclaration, NamespaceKind, ProgramRoot, StatementNode};
mod import;
mod namespace;

impl crate::ProgramNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ProgramRoot> {
        let mut errors = vec![];
        let mut statements = vec![];
        for node in &self.statement {
            node.build(ctx).append(&mut statements, &mut errors)
        }
        Success { value: ProgramRoot { statements }, diagnostics: errors }
    }
}

impl crate::StatementNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<StatementNode> {
        let value = match self {
            Self::DefineClass(v) => v.build(ctx)?.into(),
            Self::DefineEnumerate(_) => {
                todo!()
            }
            Self::DefineFunction(_) => {
                todo!()
            }
            Self::DefineImport(v) => v.build(ctx)?.into(),
            Self::DefineNamespace(v) => v.build(ctx).into(),
            Self::DefineTrait(_) => {
                todo!()
            }
            Self::DefineUnion(_) => {
                todo!()
            }
            Self::MainStatement(v) => v.build(ctx)?,
        };
        Success { value, diagnostics: vec![] }
    }
}

impl crate::MainStatementNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<StatementNode> {
        match self {
            Self::ForStatement(_) => {
                todo!()
            }
            Self::ExpressionStatement(v) => v.build(ctx).map(|v| v.into()),
            Self::WhileStatement(v) => v.build(ctx).map(|v| v.into()),
        }
    }
}

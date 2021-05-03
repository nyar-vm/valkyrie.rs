use crate::{helpers::ProgramContext, OpNamespaceNode};
use nyar_error::{Success, Validation};
use valkyrie_ast::{ExpressionNode, ImportStatement, NamespaceDeclaration, NamespaceKind, ProgramRoot, StatementNode};
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
            Self::DefineNamespace(v) => v.build(ctx).into(),
            Self::DefineImport(v) => v.build(ctx)?.into(),
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
    pub fn build(&self, ctx: &ProgramContext) -> Validation<StatementNode> {
        let expr = self.main_expression.build(ctx)?;
        let eos = self.eos.is_some();
        let ex = ExpressionNode { omit: eos, body: expr, span: self.span.clone() };
        Success { value: StatementNode::Expression(Box::new(ex)), diagnostics: vec![] }
    }
}

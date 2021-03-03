use crate::MainStatementNode;
use nyar_error::{Success, Validation};
use valkyrie_ast::{ProgramRoot, StatementNode};

impl crate::ProgramNode {
    pub fn build(&self) -> Validation<ProgramRoot> {
        let mut diagnostics = vec![];
        let mut statements = vec![];

        for node in &self.statement {}

        Success { value: ProgramRoot { statements }, diagnostics }
    }
}

impl crate::StatementNode {
    pub fn build(&self) -> Validation<StatementNode> {
        match self {
            Self::DefineClass(_) => {
                todo!()
            }
            Self::DefineFlags(_) => {
                todo!()
            }
            Self::DefineFunction(_) => {
                todo!()
            }
            Self::DefineImport(_) => {
                todo!()
            }
            Self::DefineNamespace(_) => {
                todo!()
            }
            Self::DefineTrait(_) => {
                todo!()
            }
            Self::DefineUnion(_) => {
                todo!()
            }
            Self::MainStatement(v) => v.build(),
        }
    }
}
impl crate::MainStatementNode {
    pub fn build(&self) -> Validation<StatementNode> {
        match self {
            Self::ForStatement(_) => {
                todo!()
            }
            Self::MainExpression(v) => {
                todo!()
            }
            Self::WhileStatement(_) => {
                todo!()
            }
        }
    }
}

use nyar_error::{Failure, Success, Validation};
use valkyrie_ast::{ProgramRoot, StatementNode};

impl crate::ProgramNode {
    pub fn build(&self) -> Validation<ProgramRoot> {
        let mut errors = vec![];
        let mut statements = vec![];
        for node in &self.statement {
            match node.build() {
                Success { value, diagnostics } => {
                    statements.push(value);
                    errors.extend(diagnostics)
                }
                Failure { fatal, diagnostics } => {
                    errors.push(fatal);
                    errors.extend(diagnostics)
                }
            }
        }
        Success { value: ProgramRoot { statements }, diagnostics: errors }
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
            Self::MainExpression(v) => v.build().map(|v| v.into()),
            Self::WhileStatement(_) => {
                todo!()
            }
        }
    }
}

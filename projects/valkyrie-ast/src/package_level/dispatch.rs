use super::*;

impl Display for StatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.r#type, f)?;
        if self.eos {
            f.write_str(";")?;
        }
        Ok(())
    }
}

impl Display for StatementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            StatementType::Nothing => f.write_str(";;"),
            StatementType::Namespace(v) => Display::fmt(v, f),
            StatementType::Class(v) => Display::fmt(v, f),
            StatementType::Expression(v) => Display::fmt(v, f),
            StatementType::Import(v) => Display::fmt(v, f),
            StatementType::While(v) => Display::fmt(v, f),
            StatementType::For(v) => Display::fmt(v, f),
        }
    }
}
impl From<NamespaceDeclarationNode> for StatementType {
    fn from(value: NamespaceDeclarationNode) -> Self {
        Self::Namespace(Box::new(value))
    }
}

impl From<ImportStatementNode> for StatementType {
    fn from(value: ImportStatementNode) -> Self {
        Self::Import(Box::new(value))
    }
}

impl From<ClassDeclarationNode> for StatementType {
    fn from(value: ClassDeclarationNode) -> Self {
        StatementType::Class(Box::new(value))
    }
}

impl From<WhileLoopNode> for StatementType {
    fn from(value: WhileLoopNode) -> Self {
        StatementType::While(Box::new(value))
    }
}

impl From<ForLoopNode> for StatementType {
    fn from(value: ForLoopNode) -> Self {
        StatementType::For(Box::new(value))
    }
}

impl From<ExpressionNode> for StatementType {
    fn from(value: ExpressionNode) -> Self {
        StatementType::Expression(Box::new(value))
    }
}

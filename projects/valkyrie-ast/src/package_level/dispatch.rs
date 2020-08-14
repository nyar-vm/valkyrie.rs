use super::*;
#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.r#type.build(allocator)
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementType {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            StatementType::Nothing => allocator.text(";;"),
            StatementType::Namespace(node) => node.build(allocator),
            StatementType::Import(node) => node.build(allocator),
            StatementType::Class(node) => node.build(allocator),
            StatementType::Function(node) => node.build(allocator),
            StatementType::While(node) => node.build(allocator),
            StatementType::For(node) => node.build(allocator),
            StatementType::Expression(node) => node.build(allocator),
            StatementType::Control(node) => node.build(allocator),
            StatementType::Document(node) => node.build(allocator),
            StatementType::LetBind(node) => node.build(allocator),
        }
    }
}

impl From<ControlNode> for StatementType {
    fn from(value: ControlNode) -> Self {
        StatementType::Control(Box::new(value))
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

impl From<ClassDeclaration> for StatementType {
    fn from(value: ClassDeclaration) -> Self {
        StatementType::Class(Box::new(value))
    }
}
impl From<FunctionDeclaration> for StatementType {
    fn from(value: FunctionDeclaration) -> Self {
        Self::Function(Box::new(value))
    }
}
impl From<LetBindNode> for StatementType {
    fn from(value: LetBindNode) -> Self {
        StatementType::LetBind(Box::new(value))
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

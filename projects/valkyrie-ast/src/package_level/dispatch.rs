use super::*;
#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.r#type.build(allocator)
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementBody {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            StatementBody::Nothing => allocator.text(";;"),
            StatementBody::Namespace(node) => node.build(allocator),
            StatementBody::Import(node) => node.build(allocator),
            StatementBody::Class(node) => node.build(allocator),
            StatementBody::Function(node) => node.build(allocator),
            StatementBody::While(node) => node.build(allocator),
            StatementBody::For(node) => node.build(allocator),
            StatementBody::Expression(node) => node.build(allocator),
            StatementBody::Control(node) => node.build(allocator),
            StatementBody::Document(node) => node.build(allocator),
            StatementBody::LetBind(node) => node.build(allocator),
            StatementBody::Guard(node) => node.build(allocator),
        }
    }
}

impl From<ControlNode> for StatementBody {
    fn from(value: ControlNode) -> Self {
        StatementBody::Control(Box::new(value))
    }
}

impl From<NamespaceDeclarationNode> for StatementBody {
    fn from(value: NamespaceDeclarationNode) -> Self {
        Self::Namespace(Box::new(value))
    }
}

impl From<GuardStatement> for StatementBody {
    fn from(value: GuardStatement) -> Self {
        Self::Guard(Box::new(value))
    }
}

impl From<ImportStatementNode> for StatementBody {
    fn from(value: ImportStatementNode) -> Self {
        Self::Import(Box::new(value))
    }
}

impl From<ClassDeclaration> for StatementBody {
    fn from(value: ClassDeclaration) -> Self {
        StatementBody::Class(Box::new(value))
    }
}
impl From<FunctionDeclaration> for StatementBody {
    fn from(value: FunctionDeclaration) -> Self {
        Self::Function(Box::new(value))
    }
}
impl From<LetBindNode> for StatementBody {
    fn from(value: LetBindNode) -> Self {
        StatementBody::LetBind(Box::new(value))
    }
}

impl From<WhileLoop> for StatementBody {
    fn from(value: WhileLoop) -> Self {
        StatementBody::While(Box::new(value))
    }
}

impl From<ForLoop> for StatementBody {
    fn from(value: ForLoop) -> Self {
        StatementBody::For(Box::new(value))
    }
}

impl From<ExpressionNode> for StatementBody {
    fn from(value: ExpressionNode) -> Self {
        StatementBody::Expression(Box::new(value))
    }
}

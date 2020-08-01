use super::*;
use crate::{PrettyPrint, PrettyProvider, PrettyTree};
use pretty::DocAllocator;

impl PrettyPrint for StatementNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.r#type.pretty(allocator)
    }
}

impl PrettyPrint for StatementType {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            StatementType::Nothing => allocator.text(";;"),
            StatementType::Namespace(node) => node.pretty(allocator),
            StatementType::Import(node) => node.pretty(allocator),
            StatementType::Class(node) => node.pretty(allocator),
            StatementType::Function(node) => node.pretty(allocator),
            StatementType::While(node) => node.pretty(allocator),
            StatementType::For(node) => node.pretty(allocator),
            StatementType::Expression(node) => node.pretty(allocator),
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
impl From<FunctionDeclarationNode> for StatementType {
    fn from(value: FunctionDeclarationNode) -> Self {
        Self::Function(Box::new(value))
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

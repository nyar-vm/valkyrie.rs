use super::*;

mod display;

/// The function type
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FunctionType {
    /// A function that lazy evaluate the arguments
    Macro,
    /// A function that eager evaluate the arguments
    Micro,
}

/// `class Name(Super): Trait {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionDeclarationNode {
    /// The range of the number.
    pub r#type: FunctionType,
    pub namepath: NamePathNode,
    pub modifiers: Vec<IdentifierNode>,
    pub attributes: Option<String>,
    pub arguments: ApplyArgumentNode<ExpressionTypeNode, ExpressionTermNode>,
    pub r#return: Option<ExpressionTypeNode>,
    pub body: Vec<StatementNode>,
}

/// `(args) -> return { body }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionCommonPart {
    pub generic: Option<GenericArgumentNode<ExpressionTermNode>>,
    /// The range of the number.
    pub arguments: ApplyArgumentNode<ExpressionTypeNode, ExpressionTermNode>,
    pub r#return: Option<ExpressionTypeNode>,
    pub body: Vec<StatementNode>,
}

impl FunctionDeclarationNode {}

impl FunctionCommonPart {
    /// Create a new complete function body
    #[allow(clippy::wrong_self_convention)]
    pub fn as_function(self, r#type: FunctionType, name: NamePathNode) -> FunctionDeclarationNode {
        FunctionDeclarationNode {
            r#type,
            namepath: name,
            modifiers: Vec::new(),
            attributes: None,
            arguments: self.arguments,
            r#return: self.r#return,
            body: self.body,
        }
    }
}

// impl ClassDeclare {
//     pub fn get_namepath(&self) -> Iter<'_, ValkyrieIdentifier> {
//         self.namepath.iter()
//     }
//     pub fn mut_namepath(&mut self) -> &mut Vec<ValkyrieIdentifier> {
//         &mut self.namepath
//     }
//     pub fn get_modifiers(&self) -> Iter<'_, ValkyrieIdentifier> {
//         self.modifiers.iter()
//     }
//     pub fn mut_modifiers(&mut self) -> &mut Vec<ValkyrieIdentifier> {
//         &mut self.modifiers
//     }
//     pub fn get_statement(&self) -> Iter<'_, ValkyrieASTNode> {
//         self.statements.iter()
//     }
//     pub fn mut_statement(&mut self) -> &mut Vec<ValkyrieASTNode> {
//         &mut self.statements
//     }
//     pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
//         ValkyrieASTKind::Class(box self).to_node(file, range)
//     }
// }

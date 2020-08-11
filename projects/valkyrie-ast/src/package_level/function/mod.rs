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
pub struct FunctionDeclaration {
    /// The range of the number.
    pub r#type: FunctionType,
    pub namepath: NamePathNode,
    pub modifiers: Vec<IdentifierNode>,
    pub attributes: Option<String>,
    pub generic: GenericArgumentNode,
    pub arguments: ApplyArgumentNode,
    pub r#return: Option<ExpressionNode>,
    pub body: Option<Vec<StatementNode>>,
}

/// `::<G>(args) -> return { body }`
///
/// - Auxiliary parsing function, not instantiable.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionCommonPart {
    pub generic: GenericArgumentNode,
    /// The range of the number.
    pub arguments: ApplyArgumentNode,
    pub r#return: Option<ExpressionNode>,
    pub body: Option<Vec<StatementNode>>,
}

/// `function(args) -> type := body`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionDeclarationInline {
    pub generic: GenericArgumentNode,
    /// The range of the number.
    pub arguments: ApplyArgumentNode,
    pub r#return: Option<ExpressionNode>,
    pub body: Vec<StatementNode>,
}

/// `public static final synchronized class A {}`
///
/// - Auxiliary parsing function, not instantiable.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModifierPart<'i> {
    pub modifiers: Cow<'i, [IdentifierNode]>,
}

/// `{ a; b; c }`
///
/// - Auxiliary parsing function, not instantiable.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionBodyPart<'i> {
    pub body: Cow<'i, [StatementNode]>,
}

impl<'a> FunctionBodyPart<'a> {
    pub fn new(items: Vec<StatementNode>) -> Self {
        Self { body: Cow::Owned(items) }
    }
}

impl FunctionDeclaration {
    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }
    /// Does the function has a return type
    pub fn has_return_type(&self) -> bool {
        self.r#return.is_some()
    }
    /// Does the last statement has a semicolon, or it's empty
    ///
    /// Omit return always returns `( )`
    pub fn omit_return(&self) -> bool {
        let eos: Option<bool> = try { !self.body.as_ref()?.last()?.end_semicolon };
        eos.unwrap_or(true)
    }
}

impl FunctionCommonPart {
    /// Create a new complete function body
    #[allow(clippy::wrong_self_convention)]
    pub fn as_function(self, r#type: FunctionType, name: NamePathNode) -> FunctionDeclaration {
        FunctionDeclaration {
            r#type,
            namepath: name,
            modifiers: Vec::new(),
            attributes: None,
            generic: self.generic,
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

use super::*;

mod display;

/// `micro function(args), macro procedure(args)`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    /// The belonging and name of this function
    pub namepath: NamePathNode,
    /// The range of the number.
    pub r#type: FunctionType,
    /// The annotations of this function
    pub annotations: AnnotationNode,
    /// Thy type parameters of this function
    pub generic: Option<ParametersList>,
    // The value parameters of this function
    pub arguments: ArgumentsList,
    pub r#return: Option<FunctionReturnNode>,
    pub body: StatementBlock,
}

/// `function(args) -> type := body`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionDeclarationInline {
    pub generic: ParametersList,
    /// The range of the number.
    pub arguments: ArgumentsList,
    pub r#return: Option<ExpressionNode>,
    pub body: StatementBlock,
}

/// `{ a; b; c }`
///
/// - Auxiliary parsing function, not instantiable.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementBlock {
    pub terms: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `fun name(): ReturnType`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionReturnNode {
    pub returns: ExpressionNode,
    /// The range of the node
    pub span: Range<u32>,
}

/// `fun name() / [EffectType]`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionEffectNode {
    pub effects: Vec<ExpressionNode>,
    /// The range of the node
    pub span: Range<u32>,
}

impl FunctionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FunctionType::Macro => "macro",
            FunctionType::Micro => "micro",
        }
    }
}
impl StatementBlock {
    pub fn last_semicolon(&self) -> bool {
        todo!()
    }
    pub fn fill_semicolon(&mut self) {
        todo!()
    }
}

impl FunctionDeclaration {
    /// Does the function has a return type
    pub fn has_return_type(&self) -> bool {
        self.r#return.is_some()
    }
    /// Does the last statement has a semicolon, or it's empty
    ///
    /// Omit return always returns `( )`
    pub fn omit_return(&self) -> bool {
        !self.body.last_semicolon()
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

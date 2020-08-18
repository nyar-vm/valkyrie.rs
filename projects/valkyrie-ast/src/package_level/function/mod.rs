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
    pub body: FunctionBody,
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
    pub body: FunctionBody,
}

/// `function(args) -> type := body`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionDeclarationInline {
    pub generic: GenericArgumentNode,
    /// The range of the number.
    pub arguments: ApplyArgumentNode,
    pub r#return: Option<ExpressionNode>,
    pub body: FunctionBody,
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
pub struct FunctionBody {
    pub statements: Vec<StatementNode>,
    pub span: Range<u32>,
}

impl<'i> ModifierPart<'i> {
    pub fn contains(&self, modifier: &str) -> bool {
        self.modifiers.iter().any(|x| x.name.eq(modifier))
    }
}

impl FunctionBody {
    pub fn is_empty(&self) -> bool {
        match &self.statements {
            Some(x) => x.is_empty(),
            None => true,
        }
    }
    pub fn last(&self) -> Option<&StatementNode> {
        match &self.statements {
            Some(x) => x.last(),
            None => None,
        }
    }
    pub fn fill_semicolon(&mut self) {
        match &mut self.statements {
            Some(x) => {
                for i in x.iter_mut().rev().skip(1) {
                    i.end_semicolon = true;
                }
            }
            None => {}
        }
    }
}

impl FunctionDeclaration {
    pub fn has_body(&self) -> bool {
        !self.body.is_empty()
    }
    /// Does the function has a return type
    pub fn has_return_type(&self) -> bool {
        self.r#return.is_some()
    }
    /// Does the last statement has a semicolon, or it's empty
    ///
    /// Omit return always returns `( )`
    pub fn omit_return(&self) -> bool {
        match self.body.last() {
            Some(s) => s.end_semicolon,
            None => true,
        }
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

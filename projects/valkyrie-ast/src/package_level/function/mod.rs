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
    pub generic: Option<GenericArgumentNode>,
    pub arguments: ApplyArgumentNode,
    pub r#return: Option<FunctionReturnNode>,
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

/// `fun name(): ReturnType`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionReturnNode {
    pub returns: ExpressionNode,
    pub span: Range<u32>,
}

/// `fun name() / [EffectType]`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionEffectNode {
    pub effects: Vec<ExpressionNode>,
    pub span: Range<u32>,
}

impl<'i> ModifierPart<'i> {
    pub fn contains(&self, modifier: &str) -> bool {
        self.modifiers.iter().any(|x| x.name.eq(modifier))
    }
}

impl FunctionBody {
    pub fn last_semicolon(&self) -> bool {
        match self.statements.last() {
            Some(s) => s.end_semicolon,
            None => true,
        }
    }
    pub fn fill_semicolon(&mut self) {
        for x in self.statements.iter_mut().rev().skip(1) {
            x.end_semicolon = true;
        }
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

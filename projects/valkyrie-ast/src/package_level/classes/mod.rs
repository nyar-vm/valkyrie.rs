use super::*;

#[cfg(feature = "pretty-print")]
mod display;

mod iters;

/// `class Name(Super): Trait {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDeclaration {
    /// The kind of class
    pub kind: ClassKind,
    /// The modifiers of the class.
    pub modifiers: ModifiersNode,
    /// The name of the class.
    pub identifier: IdentifierNode,
    /// The generic arguments of the class.
    pub generic: Option<GenericArgument>,
    /// The super class of the class.
    pub base_classes: Option<String>,
    /// The traits that the class implements.
    pub auto_traits: Vec<String>,
    /// The main body of the class, can't be empty.
    pub body: StatementBlock,
    /// The range of the number.
    pub span: Range<u32>,
}

/// The valid terms in a class body.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClassTerm {
    Field(ClassFieldDeclaration),
    Method(ClassMethodDeclaration),
}

/// Iterate over all legal elements in the class
#[derive(Clone, Debug)]
pub struct ClassIterator<'a> {
    inner: core::slice::Iter<'a, StatementNode>,
}

/// `field: Type = default`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassFieldDeclaration {
    /// The documentation of the node.
    pub document: DocumentationNode,
    pub modifiers: ModifiersNode,
    pub field_name: IdentifierNode,
    pub r#type: Option<ExpressionNode>,
    pub default: Option<ExpressionNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `method()`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassMethodDeclaration {
    /// The documentation of the node.
    pub document: DocumentationNode,
    /// The modifiers of the node.
    pub modifiers: ModifiersNode,
    /// `method_name()`
    pub method_name: IdentifierNode,
    /// `method_name<T>()`
    pub generic: Option<GenericArgument>,
    /// `method_name(arguments)`
    pub arguments: ApplyArgument,
    /// `: ReturnType`
    pub return_type: Option<FunctionReturnNode>,
    /// `/ EffectType`
    pub effect_type: Option<FunctionEffectNode>,
    /// `{ body }`
    pub body: Option<StatementBlock>,
}

/// `class A { }, structure V { }`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClassKind {
    /// A function that lazy evaluate the arguments
    Class,
    /// A function that eager evaluate the arguments
    Structure,
}

impl ClassKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ClassKind::Class => "class",
            ClassKind::Structure => "structure",
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

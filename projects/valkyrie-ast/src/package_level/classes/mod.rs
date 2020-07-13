use super::*;
use crate::NamePathNode;
use core::fmt::Debug;
use indentation::{IndentDisplay, IndentFormatter};

/// `class Name(Super): Trait {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDeclarationNode {
    pub namepath: NamePathNode,
    pub modifiers: Vec<IdentifierNode>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub statements: Vec<IdentifierNode>,
}

impl IndentDisplay for ClassDeclarationNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        f.write_str("class")?;

        f.write_char('}')
    }
}

impl Display for ClassDeclarationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
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

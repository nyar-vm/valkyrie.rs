use super::*;
use crate::StatementBlock;

/// `flags Bit(8bits): Trait { FlagA, FlagB }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlagsDeclaration {
    /// The documentation for this flag.
    pub documentation: DocumentationNode,
    /// `flags Name`
    pub namepath: NamePathNode,
    /// The modifiers for this flag.
    pub modifiers: Vec<IdentifierNode>,
    /// `(8bits)`
    pub layout: Option<ExpressionNode>,
    /// `: Trait`
    pub implements: Vec<String>,
    /// `{ FlagA, FlagB }`
    pub body: StatementBlock,
    /// The range of the node.
    pub span: Range<u32>,
}

/// `Name = 0x00`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlagFieldDeclaration {
    /// The documentation for this field.
    pub documentation: DocumentationNode,
    /// The identifier of the field.
    pub name: IdentifierNode,
    /// The value of the field if exists.
    pub value: Option<ExpressionNode>,
    /// The range of the node.
    pub span: Range<u32>,
}

impl PrettyPrint for FlagsDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        terms.push(allocator.keyword("flags"));
        terms.push(allocator.space());
        terms.push(self.namepath.build(allocator));
        terms.push(self.body.build(allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for FlagFieldDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        terms.push(self.name.build(allocator));
        if let Some(value) = &self.value {
            terms.push(allocator.space());
            terms.push(allocator.operator("="));
            terms.push(allocator.space());
            terms.push(value.build(allocator));
            terms.push(allocator.text(","));
        }
        allocator.concat(terms)
    }
}

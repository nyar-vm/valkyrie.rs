use super::*;
use alloc::string::ToString;

/// `union Bit(8bits): Trait {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionDeclaration {
    /// The documentation of the node.
    pub document: DocumentationNode,
    /// The range of the number.
    pub namepath: NamePathNode,
    pub modifiers: Vec<IdentifierNode>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub statements: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `VariantA { field: Type = default }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariantDeclaration {
    /// The documentation of the node.
    pub document: DocumentationNode,
    /// The range of the number.
    pub namepath: NamePathNode,
    pub modifiers: ModifiersNode,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub statements: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `field: Type = default`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldDeclaration {
    /// The documentation of the node.
    pub document: DocumentationNode,
    pub modifiers: ModifiersNode,
    pub name: IdentifierNode,
    pub r#type: Option<ExpressionNode>,
    pub default: Option<ExpressionNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `public static final synchronized class A {}`
///
/// - Auxiliary parsing function, not instantiable.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModifiersNode {
    pub terms: Vec<IdentifierNode>,
}

impl ModifiersNode {
    pub fn new(modifiers: Vec<IdentifierNode>) -> Self {
        Self { terms: modifiers }
    }
    pub fn contains(&self, modifier: &str) -> bool {
        self.terms.iter().any(|x| x.name.eq(modifier))
    }
}

impl PrettyPrint for ModifiersNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut items = Vec::with_capacity(2 * self.terms.len());
        for x in &self.terms {
            items.push(allocator.keyword(x.name.to_string()));
            items.push(allocator.space());
        }
        allocator.concat(items)
    }
}

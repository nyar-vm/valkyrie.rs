use super::*;

mod display;

/// The top level elements in script mode.
#[derive(Clone, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementNode {
    /// Placeholder for when the parser fails to parse a statement.
    Nothing,
    /// The documentation node, must have acceptor underneath.
    Document(Box<DocumentationNode>),
    /// The annotation list node.
    Annotation(Box<AnnotationList>),
    /// The namespace declaration node.
    Namespace(Box<NamespaceDeclaration>),
    /// The import statement node.
    Import(Box<ImportStatement>),
    /// The class declaration node.
    Class(Box<ClassDeclaration>),
    /// The union declaration node.
    Union(Box<UnionDeclaration>),
    /// The enumerate declaration node.
    Enumerate(Box<FlagDeclaration>),
    /// The trait declaration node
    Trait(Box<TraitDeclaration>),
    /// The type extension node
    Extends(Box<ExtendsStatement>),
    /// The function declaration node.
    Function(Box<FunctionDeclaration>),
    /// The let bind statement node.
    Variable(Box<VariableDeclaration>),
    /// The guard statement node.
    Guard(Box<GuardStatement>),
    /// The argument argument node.
    Control(Box<ControlNode>),
    /// The argument argument node.
    Expression(Box<ExpressionNode>),
}

/// The valid terms in a class body.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementContext {}

impl From<AttributeNode> for StatementNode {
    fn from(value: AttributeNode) -> Self {
        let list = AnnotationList { kind: value.kind, terms: vec![value.term], span: value.span };

        StatementNode::Annotation(Box::new(list))
    }
}

impl StatementNode {
    /// Create a new expression node
    pub fn expression(body: ExpressionType, span: Range<u32>) -> Self {
        Self::Expression(Box::new(ExpressionNode { omit: false, body, span: span.clone() }))
    }
    /// Create a new raw text node
    pub fn text<S: ToString>(s: S, span: Range<u32>) -> Self {
        let literal = StringTextNode { text: s.to_string(), span: span.clone() };
        Self::expression(ExpressionType::Text(Box::new(literal)), span)
    }
}

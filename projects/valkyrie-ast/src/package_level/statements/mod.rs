use super::*;

mod display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementNode {
    pub r#type: StatementType,
    pub end_semicolon: bool,
}

/// The top level elements in script mode.
#[derive(Clone, Debug, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementType {
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
    /// The class's declaration node.
    ClassField(Box<ClassFieldDeclaration>),
    /// The class's method declaration node.
    ClassMethod(Box<ClassMethodDeclaration>),
    /// The union declaration node.
    Union(Box<UnionDeclaration>),
    /// The union's field declaration node.
    UnionField(Box<UnionFieldDeclaration>),
    /// The enumerate declaration node.
    Enumerate(Box<EnumerateDeclaration>),
    /// The enumerates field declaration node.
    EnumerateField(Box<EnumerateFieldDeclaration>),
    /// The flags declaration node.
    Flags(Box<FlagsDeclaration>),
    /// The tagged union declaration node.
    Tagged(Box<TaggedDeclaration>),
    /// The tagged union's variant declaration node.
    Variant(Box<VariantDeclaration>),
    /// The function declaration node.
    Function(Box<FunctionDeclaration>),
    /// The while loop statement node.
    While(Box<WhileLoop>),
    /// The for loop statement node.
    For(Box<ForLoop>),
    /// The let bind statement node.
    LetBind(Box<LetBindNode>),
    /// The guard statement node.
    Guard(Box<GuardStatement>),
    /// The apply argument node.
    Control(Box<ControlNode>),
    /// The apply argument node.
    Expression(Box<ExpressionNode>),
}

/// The valid terms in a class body.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementContext {}

impl From<AnnotationNode> for StatementType {
    fn from(value: AnnotationNode) -> Self {
        let list = AnnotationList { kind: value.kind, terms: vec![value.term], span: value.span };

        StatementType::Annotation(Box::new(list))
    }
}

impl StatementNode {
    /// Create a new expression node
    pub fn expression(body: ExpressionType, span: Range<u32>) -> Self {
        Self {
            r#type: StatementType::Expression(Box::new(ExpressionNode { type_level: false, body, span: span.clone() })),
            end_semicolon: false,
        }
    }
    /// Create a new raw text node
    pub fn text<S: ToString>(s: S, span: Range<u32>) -> Self {
        let literal = StringTextNode { text: s.to_string(), span: span.clone() };
        Self::expression(ExpressionType::Text(Box::new(literal)), span)
    }
    pub fn string<S: ToString>(s: S, span: Range<u32>) -> Self {
        let literal = StringLiteralNode { literal: s.to_string(), handler: None, span: span.clone() };
        Self::expression(ExpressionType::String(Box::new(literal)), span)
    }
}

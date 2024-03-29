use super::*;

mod display;

/// The kind of trait
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TraitKind {
    /// `trait`
    Trait,
    /// `interface`
    Interface,
}

/// `trait name: Debug {}`
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TraitDeclaration {
    /// The kind of the trait
    pub kind: TraitKind,
    /// The name of the trait
    pub name: IdentifierNode,
    /// The generic parameters
    pub generics: Option<ParametersList>,
    /// `trait A: Debug { }`, the trait bounds
    pub implements: Option<ExpressionKind>,
    /// the needed fields(zero parameter method, get + set)
    pub body: Vec<TraitTerm>,
}

/// `extends path::A: Debug {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendsStatement {
    /// `extends A: Debug { }`, the trait bounds
    pub implements: Option<ExpressionKind>,
    /// The additional methods
    pub body: Vec<TraitTerm>,
}

/// The valid terms in a trait body.
#[derive(Clone, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TraitTerm {
    /// `@expand {}`
    Macro(ProceduralNode),
    /// `field: Type = default`
    Field(FieldDeclaration),
    /// `method()`
    Method(MethodDeclaration),
}

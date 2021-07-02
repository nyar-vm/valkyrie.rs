use super::*;

mod display;
mod iters;

/// `import package::module::path`
/// `import "external"`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportStatement {
    /// The annotation of the import
    pub annotation: AttributeList,
    /// The term of the import
    pub term: ImportTermNode,
    /// The range of the node
    pub span: Range<u32>,
}

/// A valid import term of the import statement
#[derive(Clone, Debug, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportTermNode {
    /// `a.b.c.{ ... }`
    Group(ImportGroupNode),
    /// `a.b.c.*`
    All(ImportAllNode),
    /// `a.b.c as alias`
    Alias(ImportAliasNode),
}

impl Default for ImportTermNode {
    fn default() -> Self {
        Self::Group(Default::default())
    }
}

/// `path { group }`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportGroupNode {
    /// The path of the import
    pub path: Vec<IdentifierNode>,
    /// The group of the import
    pub group: Vec<ImportTermNode>,
}

/// `import package∷module.*`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportAllNode {
    /// The path of the import
    pub path: Vec<IdentifierNode>,
}

/// `path as alias`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportAliasNode {
    /// The path of the import
    pub path: Vec<IdentifierNode>,
    /// The item of the import
    pub item: ImportAliasItem,
    /// The alias of the import
    pub alias: Option<ImportAliasItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportAliasItem {
    Attribute(IdentifierNode),
    Procedural(IdentifierNode),
    Normal(IdentifierNode),
}

/// A resolved import item
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImportResolvedItem {
    /// The annotation of the import
    pub annotation: Option<Arc<AttributeList>>,
    /// The path of the import
    pub path: Vec<IdentifierNode>,
    /// The alias of the import
    pub alias: Option<IdentifierNode>,
}

/// The resolve result of import
///
/// - `function`, `constant`, `structure`, `interfaces`, `type` share a namespace
/// - `derive` and `macro` share a namespace
/// - If a macro is an implicit macro, it occupies the namespace of the function
///
/// ```vk
/// namespace a {
///     class A;
///     class B;
///     class C;
///     class D;
/// }
///
/// namespace b {
///     class A;
///     class B;
///     class C;
/// }
///
/// using a.*;
/// using b.*;
/// using a.{A, B};
/// using b.B;
/// ```
///
/// ## Script Mode
///
/// In script mode, adjusting the import statement orders at the same level does not affect the final result.
/// - You cannot explicitly import two objects with the same name, it is a compile-time error
/// - You cannot implicitly import two objects with the same name, this is a compile-time warning
/// - Extensions cannot be implicitly imported
///
/// - A: `a::A (explicit)`
/// - B: `null (duplicate, error, not available)`
/// - C: `null (ambiguous, waring, not available)`
/// - D: `a::D (implicit)`
///
/// ## Interactive Mode
///
/// In repl mode, imports always available.
/// - later implicit imports will override earlier implicit imports
/// - later explicit imports will override earlier explicit imports
///
/// - A: `a::A (explicit)`
/// - B: `b::B (explicit)`
/// - C: `b::C (implicit)`
/// - D: `a::D (implicit)`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportState {
    /// Available by explicit import
    Explicit,
    /// Available by implicit import
    Implicit,
    /// Unavailable due to duplicate import
    Duplicate,
    /// Unavailable due to ambiguous import
    Ambiguous,
}

impl ImportResolvedItem {
    /// Add external identifiers
    pub fn join_external(&self, name: &IdentifierNode) -> Self {
        todo!()
        // Self { annotation: Some(Arc::new(name.clone())), ..self.clone() }
    }
    /// Join import names
    pub fn join_name(&self, name: &IdentifierNode) -> Self {
        let mut path = self.path.clone();
        path.push(name.clone());
        Self { path, ..self.clone() }
    }
    /// Join a path to the import
    pub fn join_path(&self, namepath: &[IdentifierNode]) -> Self {
        let mut path = self.path.clone();
        path.extend_from_slice(namepath);
        Self { path, ..self.clone() }
    }
    /// Join an alias to the import
    pub fn join_alias(&self, alias: &IdentifierNode) -> Self {
        Self { alias: Some(alias.clone()), ..self.clone() }
    }
}

impl ImportStatement {
    /// Create a new import statement
    pub fn flatten(&self) -> Vec<ImportResolvedItem> {
        let root = ImportResolvedItem::default();
        let mut all = Vec::new();
        self.term.resolve(&root, &mut all);
        all
    }
}

impl ImportTermNode {
    /// Resolve the import term
    fn resolve(&self, parent: &ImportResolvedItem, all: &mut Vec<ImportResolvedItem>) {
        match self {
            ImportTermNode::Alias(alias) => {
                // all.push(parent.join_path(&alias.path.names).join_alias(&alias.alias));
            }
            ImportTermNode::Group(group) => {
                // let root = parent.join_path(&group.path.names);
                // for item in &group.group {
                //     item.resolve(&root, all);
                // }
            }
            ImportTermNode::All(all) => {}
        }
    }
}

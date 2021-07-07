use super::*;
use crate::helper::IdentifiersDisplay;
use alloc::rc::Rc;
use nyar_error::FileSpan;

mod display;
mod iters;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportKind {
    /// `import!`, share import item in all files under same namespace
    Shared,
    /// `import`, only usable in this file
    Private,
    /// `import* `, enable only under the debug mode
    Test,
}

/// `import package::module::path`
/// `import "external"`
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportStatement {
    /// The annotation of the import
    pub annotation: AttributeList,
    /// The important kind
    pub kind: ImportKind,
    /// The term of the import
    pub term: ImportTermNode,
    /// The range of the node
    pub span: FileSpan,
}

/// A valid import term of the import statement
#[derive(Clone, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportTermNode {
    /// `a.b.c.{ ... }`
    Group(ImportGroupNode),
    /// `a.b.c.*`
    All(ImportAllNode),
    /// `a.b.c as alias`
    Alias(ImportAliasNode),
}

/// `group.path { item1, item2 }`
#[derive(Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportGroupNode {
    /// The path of the import
    pub path: Vec<IdentifierNode>,
    /// The group of the import
    pub terms: Vec<ImportTermNode>,
}

/// `import package∷module.*`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportAllNode {
    /// The path of the import
    pub path: Vec<IdentifierNode>,
    pub span: Range<u32>,
}

impl ValkyrieNode for ImportAllNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}

/// `path as alias`
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportAliasNode {
    /// The path of the import
    pub path: Vec<IdentifierNode>,
    /// The item of the import
    pub item: ImportAliasItem,
    /// The alias of the import
    pub alias: Option<ImportAliasItem>,
}

/// The name of import items
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportAliasItem {
    /// `#attribute`
    Attribute(IdentifierNode),
    /// `@procedural`
    Procedural(IdentifierNode),
    /// `name`
    Normal(IdentifierNode),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImportResolvedKind {
    Normal,
    All,
    Alias(IdentifierNode),
}

impl Default for ImportResolvedKind {
    fn default() -> Self {
        Self::Normal
    }
}

/// A resolved import item
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ImportResolvedItem {
    /// The path of the import
    pub path: Vec<Box<str>>,
    /// The alias of the import
    pub kind: ImportResolvedKind,
    /// The position fo the resolved item
    pub span: FileSpan,
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
    }
    /// Join import names
    pub fn join_name(&self, name: &IdentifierNode) -> Self {
        todo!()
    }
    /// Join a path to the import
    pub fn join_path(&self, namepath: &[IdentifierNode]) -> Self {
        todo!()
    }
    /// Join an alias to the import
    fn join_alias(&self, alias: &ImportAliasNode) -> Self {
        todo!()
    }
    fn join_all(mut self, alias: &ImportAllNode) -> Self {
        self.span.set_range(alias.get_range());
        self.path.extend(alias.path.iter().map(|s| Box::from(s.name.as_str())));
        self.kind = ImportResolvedKind::All;
        self
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
                all.push(parent.clone().join_alias(alias));
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

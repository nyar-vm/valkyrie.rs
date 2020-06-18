use super::*;

mod display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableKind {
    /// `(a, b, c)`
    Tuple,
    /// `{a, b, c}`
    OffsetTable,
    /// `{a = 1, b = 2, c = 3}`
    OrdinalTable,
}

/// A number literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableNode<E> {
    pub kind: TableKind,
    /// The raw string of the number.
    pub terms: Vec<CallTermNode<IdentifierNode, E>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// A number literal.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallTermNode<K, V> {
    pub key: Option<K>,
    pub value: V,
}

impl<K, V> Display for CallTermNode<K, V>
where
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.key {
            Some(ref k) => write!(f, "{}: {}", k, self.value),
            None => Display::fmt(&self.value, f),
        }
    }
}

/// `function(0, a: Type, b: Integer = 1)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentTermNode<K, V, D> {
    pub key: Option<K>,
    pub value: V,
    pub default: Option<D>,
}

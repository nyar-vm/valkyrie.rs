pub enum TableKind {
    Tuple,
    Table,
}

/// A number literal.
#[derive(Debug, Clone, Eq)]
pub struct TableNode {
    pub index0: bool,
    /// The raw string of the number.
    pub terms: Vec<TableTermNode>,
    /// The range of the number.
    pub range: Range<usize>,
}
/// A number literal.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TableTermNode {
    /// `array[index]`, also can be a call_index `array[[1, 2, 3]]`
    Item(ValkyrieExpression),
    /// `a[start:end:step]`
    Pair(PairNode),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PairNode {
    pub key: IdentifierNode,
    pub value: ValkyrieExpression,
}

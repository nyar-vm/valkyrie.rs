use super::*;

impl Display for TableKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.begin_str(), self.end_str())
    }
}

impl TableKind {
    fn begin_str(&self) -> &'static str {
        match self {
            TableKind::Tuple => "(",
            TableKind::OffsetTable => "[",
            TableKind::OrdinalTable => "[",
        }
    }
    fn end_str(&self) -> &'static str {
        match self {
            TableKind::Tuple => ")",
            TableKind::OffsetTable => "]",
            TableKind::OrdinalTable => "]",
        }
    }
}

impl<E: Display> Display for TableNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.begin_str())?;
        for (i, term) in self.terms.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            match term {
                TableTermNode::Item(item) => write!(f, "{}", item)?,
                TableTermNode::Pair(pair) => write!(f, "{}: {}", pair.key, pair.value)?,
            }
        }
        write!(f, "{}", self.kind.end_str())
    }
}

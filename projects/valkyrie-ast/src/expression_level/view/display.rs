use super::*;

impl<E: Display> Display for ViewNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(if self.index0 { '[' } else { '⁅' })?;
        for (idx, item) in self.terms.iter().enumerate() {
            if idx != 0 {
                f.write_str(", ")?;
            }
            Display::fmt(item, f)?;
        }
        f.write_char(if self.index0 { ']' } else { '⁆' })
    }
}

impl<E: Display> Display for ViewTermNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewTermNode::Index(v) => Display::fmt(v, f),
            ViewTermNode::Range(v) => Display::fmt(v, f),
        }
    }
}

impl<E> Display for ViewRangeNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

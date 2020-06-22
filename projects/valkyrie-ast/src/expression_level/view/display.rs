use super::*;

impl<E: Display> Display for ViewNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.base, f)?;
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

impl<E: Display> Display for ViewRangeNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(start) = &self.start {
            Display::fmt(start, f)?;
        }
        f.write_char(':')?;
        match &self.end {
            Some(end) => {
                Display::fmt(end, f)?;
                f.write_char(':')?;
            }
            None => f.write_str(" :")?,
        }

        if let Some(step) = &self.step {
            Display::fmt(step, f)?;
        }
        Ok(())
    }
}

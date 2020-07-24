use super::*;

impl<E: IndentDisplay> IndentDisplay for ViewNode<E> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        self.base.indent_fmt(f)?;
        f.write_char(if self.index0 { '[' } else { '⁅' })?;
        for (idx, item) in self.terms.iter().enumerate() {
            if idx != 0 {
                f.write_str(", ")?;
            }
            item.indent_fmt(f)?;
        }
        f.write_char(if self.index0 { ']' } else { '⁆' })
    }
}

impl<E: IndentDisplay> IndentDisplay for ViewTermNode<E> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        match self {
            ViewTermNode::Index(v) => v.indent_fmt(f),
            ViewTermNode::Range(v) => v.indent_fmt(f),
        }
    }
}
impl<E: IndentDisplay> IndentDisplay for ViewRangeNode<E> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        if let Some(start) = &self.start {
            start.indent_fmt(f)?;
        }
        f.write_char(':')?;
        match &self.end {
            Some(end) => {
                end.indent_fmt(f)?;
                f.write_char(':')?;
            }
            None => f.write_str(" :")?,
        }
        if let Some(step) = &self.step {
            step.indent_fmt(f)?;
        }
        Ok(())
    }
}

impl<E: IndentDisplay> Display for ViewTermNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl<E: IndentDisplay> Display for ViewNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl<E: IndentDisplay> Display for ViewRangeNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

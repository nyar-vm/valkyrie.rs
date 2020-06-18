use super::*;

impl<E: Display> Display for ApplyCallNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.base)?;
        for (i, v) in self.terms.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            Display::fmt(v, f)?;
        }
        write!(f, ")")
    }
}

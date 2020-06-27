use super::*;

impl<E: Display> Display for ApplyDotNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}(", self.base, self.caller)?;
        for (i, v) in self.terms.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            Display::fmt(v, f)?;
        }
        write!(f, ")")
    }
}

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

impl<K, V> Display for ApplyTermNode<K, V>
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

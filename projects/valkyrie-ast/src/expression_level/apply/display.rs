use super::*;
use indentation::{IndentDisplay, IndentFormatter};

impl<E: Display> Display for ApplyDotNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}(", self.base, self.caller)?;
        comma_terms(f, &self.terms)?;
        write!(f, ")")
    }
}

impl<E: Display> IndentDisplay for ApplyCallNode<E> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<E: Display> Display for ApplyCallNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}(", self.base)?;
        comma_terms(f, &self.terms)?;
        write!(f, ")")
    }
}

impl<K, V> Display for ApplyTermNode<K, V>
where
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.key {
            Some(ref k) => write!(f, "{}: {}", k, self.value),
            None => Display::fmt(&self.value, f),
        }
    }
}

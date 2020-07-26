use super::*;

impl IndentDisplay for ApplyDotNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        self.base.indent_fmt(f)?;
        f.write_newline()?;
        write!(f, ".{}(", self.caller)?;
        comma_terms(f, &self.terms)?;
        write!(f, ")")
    }
}

impl<K, V> IndentDisplay for ApplyTermNode<K, V>
where
    K: IndentDisplay,
    V: IndentDisplay,
{
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        match &self.key {
            Some(k) => {
                k.indent_fmt(f)?;
                f.write_str(": ")?;
                self.value.indent_fmt(f)
            }
            None => self.value.indent_fmt(f),
        }
    }
}

impl IndentDisplay for ApplyCallNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        self.base.indent_fmt(f)?;
        write!(f, "(")?;
        comma_terms(f, &self.terms)?;
        write!(f, ")")
    }
}

impl<K, V> Display for ApplyTermNode<K, V>
where
    K: IndentDisplay,
    V: IndentDisplay,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl<K: Display, V: Display, D: Display> Display for ArgumentTermNode<K, V, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.key)?;
        if let Some(value) = &self.value {
            write!(f, ": {}", value)?;
        }
        if let Some(default) = &self.default {
            write!(f, " = {}", default)?;
        }
        Ok(())
    }
}

impl IndentDisplay for ArgumentKeyNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        for modifier in &self.modifiers {
            write!(f, "{} ", modifier)?;
        }
        write!(f, "{}", self.key)
    }
}

impl Display for ArgumentKeyNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl Display for ApplyDotNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl Display for ApplyCallNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

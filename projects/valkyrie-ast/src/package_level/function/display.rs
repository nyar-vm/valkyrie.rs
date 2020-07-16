use super::*;
use crate::utils::comma_terms;

impl IndentDisplay for FunctionDeclarationNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        write!(f, "{} {}", self.r#type, self.namepath)?;
        f.write_char('(')?;
        f.indent();
        for term in self.arguments.terms.iter() {
            f.write_newline()?;
            term.indent_fmt(f)?;
            f.write_str(",")?;
        }
        f.dedent();
        f.write_newline()?;
        f.write_char(')')?;

        f.write_char('{')?;
        f.indent();
        for term in self.body.iter() {
            f.write_newline()?;
            term.indent_fmt(f)?;
            f.write_str(";")?;
        }
        f.dedent();
        f.write_newline()?;
        f.write_char('}')
    }
}

impl<K: Display, V: Display, D: Display> IndentDisplay for ArgumentTermNode<K, V, D> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
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

impl Display for FunctionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            FunctionType::Macro => f.write_str("macro"),
            FunctionType::Micro => f.write_str("micro"),
        }
    }
}

impl Display for FunctionDeclarationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

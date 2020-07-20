use super::*;
use crate::utils::comma_terms;
use core::borrow::BorrowMut;

impl IndentDisplay for FunctionDeclarationNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        write!(f, "{} {}", self.r#type, self.namepath)?;
        if let Some(s) = &self.generic {
            Display::fmt(s, f.borrow_mut())?
        }

        f.write_char('(')?;
        comma_terms(f.borrow_mut(), &self.arguments.terms)?;
        f.write_char(')')?;
        if let Some(ret) = &self.r#return {
            write!(f, ": {ret}")?
        }
        if let Some(s) = &self.body {
            f.write_str(" {")?;
            f.indent();
            for term in s.iter() {
                f.write_newline()?;
                term.indent_fmt(f)?;
                f.write_str(";")?;
            }
            f.dedent();
            f.write_newline()?;
            f.write_char('}')?;
        }
        Ok(())
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

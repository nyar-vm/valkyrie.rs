use super::*;
use indentation::{IndentDisplay, IndentFormatter};

impl Display for ConditionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ConditionType::AlwaysTrue => Ok(()),
            ConditionType::Case => write!(f, "case"),
            ConditionType::Expression(e) => write!(f, "if {}", e),
        }
    }
}

impl Display for WhileLoopNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "while {} {{\n    ", self.condition)?;
        for i in &self.body {
            write!(f, "{}\n    ", i)?;
        }
        write!(f, "}}")?;
        if !self.r#else.is_empty() {
            write!(f, " else {{\n    ")?;
            for i in &self.r#else {
                write!(f, "{}\n    ", i)?;
            }
            write!(f, "}}")?;
        }
        Ok(())
    }
}

impl Display for ForLoopNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for i in &self.body {
            write!(f, "{}\n", i)?;
        }
        write!(f, "}}")?;
        if !self.r#else.is_empty() {
            write!(f, " else {{\n")?;
            for i in &self.r#else {
                write!(f, "{}\n", i)?;
            }
            write!(f, "}}")?;
        }
        Ok(())
    }
}

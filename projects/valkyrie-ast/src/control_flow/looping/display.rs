use super::*;
use crate::{ApplyCallNode, ExpressionType, StatementType};
use core::fmt::Write;
use indentation::{IndentDisplay, IndentFormatter};

impl Display for ConditionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ConditionType::AlwaysTrue => f.write_str("true"),
            ConditionType::Case => f.write_str("case"),
            ConditionType::Expression(e) => Display::fmt(e, f),
        }
    }
}

impl IndentDisplay for ExpressionNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        self.expression.indent_fmt(f)
    }
}

impl IndentDisplay for ExpressionType {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        match self {
            ExpressionType::Placeholder => {
                todo!()
            }
            ExpressionType::Symbol(_) => {
                todo!()
            }
            ExpressionType::Number(_) => {
                todo!()
            }
            ExpressionType::String(_) => {
                todo!()
            }
            ExpressionType::Prefix(_) => {
                todo!()
            }
            ExpressionType::Binary(_) => {
                todo!()
            }
            ExpressionType::Suffix(_) => {
                todo!()
            }
            ExpressionType::Table(_) => {
                todo!()
            }
            ExpressionType::Apply(v) => v.indent_fmt(f),
            ExpressionType::ApplyDot(_) => {
                todo!()
            }
            ExpressionType::View(_) => {
                todo!()
            }
            ExpressionType::GenericCall(_) => {
                todo!()
            }
        }
    }
}

impl<E: Display> IndentDisplay for ApplyCallNode<E> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl IndentDisplay for WhileLoopNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        write!(f, "while {} {{", self.condition)?;
        f.indent();
        for node in self.body.iter() {
            f.write_newline()?;
            node.indent_fmt(f)?;
        }
        f.dedent();
        f.write_newline()?;
        f.write_char('}')?;
        if self.r#else.is_empty() {
            return Ok(());
        }
        write!(f, " else {{\n    ")?;
        for i in &self.r#else {
            write!(f, "{}\n    ", i)?;
        }
        f.write_newline()?;
        f.write_char('}')
    }
}

impl Display for WhileLoopNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
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

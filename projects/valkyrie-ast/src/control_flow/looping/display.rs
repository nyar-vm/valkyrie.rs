use super::*;

impl Display for ConditionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ConditionType::AlwaysTrue => f.write_str("true"),
            ConditionType::Case => f.write_str("case"),
            ConditionType::Expression(e) => Display::fmt(e, f),
        }
    }
}

impl<T: IndentDisplay> IndentDisplay for ExpressionNode<T> {
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
            ExpressionType::Symbol(v) => {
                write!(f, "{}", v)
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
        format_else_body(f, &self.r#else)
    }
}

impl IndentDisplay for ForLoopNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        for i in &self.body {
            write!(f, "{}\n", i)?;
        }
        f.write_newline()?;
        f.write_char('}')?;
        format_else_body(f, &self.r#else)
    }
}

impl Display for WhileLoopNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl Display for ForLoopNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

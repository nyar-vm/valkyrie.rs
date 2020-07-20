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

impl<const T: ExpressionContext> IndentDisplay for ExpressionNode<T> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        self.body.indent_fmt(f)
    }
}

impl IndentDisplay for ExpressionBody {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        match self {
            ExpressionBody::Placeholder => f.write_str("???"),
            ExpressionBody::Symbol(v) => Display::fmt(v, f.borrow_mut()),
            ExpressionBody::Number(v) => Display::fmt(v, f.borrow_mut()),
            ExpressionBody::String(v) => Display::fmt(v, f.borrow_mut()),
            ExpressionBody::Prefix(v) => Display::fmt(v, f.borrow_mut()),
            ExpressionBody::Binary(v) => Display::fmt(v, f.borrow_mut()),
            ExpressionBody::Suffix(v) => Display::fmt(v, f.borrow_mut()),
            ExpressionBody::Table(v) => Display::fmt(v, f.borrow_mut()),
            ExpressionBody::Apply(v) => v.indent_fmt(f),
            ExpressionBody::ApplyDot(v) => Display::fmt(v, f.borrow_mut()),
            ExpressionBody::View(v) => Display::fmt(v, f.borrow_mut()),
            ExpressionBody::GenericCall(v) => Display::fmt(v, f.borrow_mut()),
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

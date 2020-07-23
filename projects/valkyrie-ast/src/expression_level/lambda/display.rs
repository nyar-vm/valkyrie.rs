use super::*;

impl IndentDisplay for LambdaCallNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        self.base.indent_fmt(f)?;
        f.write_str(" {")?;
        f.indent();
        f.write_newline()?;
        for statement in &self.body {
            statement.indent_fmt(f)?;
        }
        f.dedent();
        f.write_newline()?;
        f.write_str("}")
    }
}

impl IndentDisplay for LambdaDotNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        self.base.indent_fmt(f)?;
        f.indent();
        f.write_newline()?;
        f.write_str(".{")?;
        f.indent();
        f.write_newline()?;
        for statement in &self.body {
            statement.indent_fmt(f)?;
        }
        f.dedent();
        f.write_newline()?;
        f.write_str("}")
    }
}

impl Display for LambdaCallNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl Display for LambdaDotNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

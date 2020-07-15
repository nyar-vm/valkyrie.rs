use super::*;

impl IndentDisplay for FunctionDeclarationNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        write!(f, "{} {}", self.r#type, self.namepath)?;

        f.write_char('}')
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

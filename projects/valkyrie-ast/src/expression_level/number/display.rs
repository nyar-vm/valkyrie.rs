use super::*;

impl IndentDisplay for NumberLiteralNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        match &self.unit {
            Some(s) => write!(f, "{}{}", self.value, s),
            None => write!(f, "{}", self.value),
        }
    }
}

wrap_display!(NumberLiteralNode);

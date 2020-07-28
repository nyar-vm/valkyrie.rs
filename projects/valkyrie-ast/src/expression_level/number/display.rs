use super::*;

impl IndentDisplay for NumberLiteralNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        match &self.unit {
            Some(s) => write!(f, "{}{}", self.value, s),
            None => write!(f, "{}", self.value),
        }
    }
}

impl PrettyPrint for NumberLiteralNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> RefDoc<'a, ColorSpec> {
        let num = allocator.text(self.value.to_string()).annotate(allocator.number_style());
        match &self.unit {
            Some(s) => {
                let unit = allocator.text(s.name.to_string()).annotate(allocator.macro_style());
                num.append(unit).into_doc()
            }
            None => num.into_doc(),
        }
    }
}

wrap_display!(NumberLiteralNode);

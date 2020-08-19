use super::*;

impl Display for StringLiteralNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if let Some(unit) = &self.unit {
            f.write_str(&unit.name)?;
        }
        f.write_char('\'')?;

        for c in self.value.chars() {
            match c {
                '\n' => f.write_str("\\n")?,
                '\r' => f.write_str("\\r")?,
                '\t' => f.write_str("\\t")?,
                '\\' => f.write_str("\\\\")?,
                '\'' => f.write_str("\\'")?,
                _ => f.write_char(c)?,
            }
        }
        f.write_char('\'')
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for StringLiteralNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.string(self.to_string())
    }
}

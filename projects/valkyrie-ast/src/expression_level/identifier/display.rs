use super::*;

impl Display for IdentifierNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl Display for NamepathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, item) in self.names.iter().enumerate() {
            if idx != 0 {
                f.write_str("∷")?;
            }
            Display::fmt(item, f)?;
        }
        Ok(())
    }
}

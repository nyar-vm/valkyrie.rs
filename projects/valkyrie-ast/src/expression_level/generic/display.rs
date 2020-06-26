use super::*;

impl<E: Display> Display for GenericCall<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)?;
        if !self.terms.is_empty() {
            write!(f, "⦓")?;
            for (i, term) in self.terms.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", term)?;
            }
            write!(f, "⦔")?;
        }
        Ok(())
    }
}

use super::*;

impl<E: Display> Display for GenericCall<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)?;
        write!(f, "⦓")?;
        comma_terms(f, &self.terms)?;
        write!(f, "⦔")?;
        Ok(())
    }
}

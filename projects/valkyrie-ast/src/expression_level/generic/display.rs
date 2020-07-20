use super::*;

impl Display for GenericCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.base)?;
        write!(f, "⦓")?;
        comma_terms(f, &self.terms)?;
        write!(f, "⦔")?;
        Ok(())
    }
}

impl Display for GenericArgumentNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "⦓")?;
        comma_terms(f, &self.terms)?;
        write!(f, "⦔")?;
        Ok(())
    }
}

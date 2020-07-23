use super::*;

impl IndentDisplay for GenericCall {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        write!(f, "{}", self.base)?;
        write!(f, "⦓")?;
        comma_terms(f, &self.terms)?;
        write!(f, "⦔")
    }
}

impl IndentDisplay for GenericArgumentNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        write!(f, "⦓")?;
        comma_terms(f, &self.terms)?;
        write!(f, "⦔")
    }
}

impl Display for GenericCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl Display for GenericArgumentNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

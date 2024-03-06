use super::*;

impl PartialEq<str> for ArgumentTerm {
    fn eq(&self, other: &str) -> bool {
        self.key.eq(other)
    }
}

impl PartialEq<str> for ArgumentKey {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::Nothing => false,
            Self::Symbol(s) => s.name.eq(other),
        }
    }
}

impl AddAssign<ArgumentTerm> for ArgumentsList {
    fn add_assign(&mut self, rhs: ArgumentTerm) {
        self.terms.push(rhs)
    }
}

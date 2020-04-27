use super::*;

impl Debug for ValkyrieInfix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Infix({}, {:?})", self.as_operator(), self.range)
    }
}

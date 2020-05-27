use super::*;

impl Display for NumberNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.unit {
            Some(s) => write!(f, "{}{}", self.value, s),
            None => write!(f, "{}", self.value),
        }
    }
}

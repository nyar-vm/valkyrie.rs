use std::borrow::Cow;
use std::fmt::Write;
use std::str::Chars;
use crate::helpers::StringRewrite;
use super::*;


impl ValkyrieIdentifier {
    pub fn get_identifier(&self) -> Cow<str> {
        if self.name.starts_with('`') {
            Cow::Owned(StringRewrite::view(&self.name, 1, self.name.len() - 1).collect())
        } else {
            Cow::Borrowed(&self.name)
        }
    }
    pub fn get_safe_name(&self) -> &str {
        if self.name.starts_with('`') {
            &self.name[1..self.name.len() - 1]
        } else {
            &self.name
        }
    }
}

impl Display for ValkyrieIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_identifier())
    }
}


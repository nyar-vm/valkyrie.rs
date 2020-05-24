use std::borrow::Cow;
use valkyrie_ast::ValkyrieIdentifier;

use super::*;
use crate::helpers::StringRewrite;

impl ValkyrieIdentifier {
    pub fn get_identifier(&self) -> Cow<str> {
        if self.name.starts_with('`') {
            Cow::Owned(StringRewrite::view(&self.name, 1, self.name.len() - 1).collect())
        }
        else {
            Cow::Borrowed(&self.name)
        }
    }
    pub fn get_safe_name(&self) -> &str {
        if self.name.starts_with('`') { &self.name[1..self.name.len() - 1] } else { &self.name }
    }
}

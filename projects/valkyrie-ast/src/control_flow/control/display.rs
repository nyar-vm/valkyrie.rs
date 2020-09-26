use super::*;

impl Display for ControlType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PrettyPrint for RaiseNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(2);
        terms.push(allocator.keyword("raise"));
        terms.push(allocator.space());
        if let Some(s) = &self.expression {
            terms.push(s.build(allocator));
        }
        allocator.concat(terms)
    }
}

impl PrettyPrint for ControlNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        terms.push(self.r#type.build(allocator));
        if let Some(s) = &self.expression {
            terms.push(allocator.space());
            terms.push(s.build(allocator));
        }
        allocator.concat(terms)
    }
}

impl PrettyPrint for ControlType {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.keyword(self.as_str())
    }
}

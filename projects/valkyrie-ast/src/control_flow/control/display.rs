use super::*;

impl ControlType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ControlType::Break => "break",
            ControlType::Continue => "continue",
            ControlType::Fallthrough => "fallthrough",
            ControlType::Return => "return",
            ControlType::Resume => "resume",
            ControlType::Raise => "raise",
            ControlType::YieldReturn => "yield",
            ControlType::YieldBreak => "yield break",
            ControlType::YieldFrom => "yield from",
        }
    }
}

impl Display for ControlType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
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

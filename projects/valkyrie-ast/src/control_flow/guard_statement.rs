use super::*;

/// `guard ... else { ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GuardStatement {
    pub condition: GuardType,
    pub body: FunctionBody,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GuardType {
    Case,
    Inline(Box<ExpressionNode>),
    // Block(Box<FunctionBody>),
}

impl PrettyPrint for GuardStatement {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(10);
        terms.push(allocator.keyword("guard"));
        terms.push(allocator.space());
        match &self.condition {
            GuardType::Inline(e) => {
                terms.push(e.build(allocator));
                terms.push(allocator.space());
            }
            // GuardType::Block(s) => terms.push(s.build(allocator)),
            GuardType::Case => {
                todo!()
            }
        }
        terms.push(allocator.keyword("else"));
        terms.push(self.body.build(allocator));
        allocator.concat(terms)
    }
}

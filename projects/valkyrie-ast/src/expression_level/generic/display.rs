use super::*;
use crate::PrettyTree;

impl PrettyPrint for GenericCall {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = self.base.build(allocator);
        let lhs = allocator.text("⦓").append(allocator.softline());
        let rhs = allocator.softline().append(allocator.text("⦔"));
        let body = self.terms.iter().map(|x| x.build(allocator).append(allocator.softline()));
        head.append(lhs).append(allocator.concat(body)).append(rhs)
    }
}

impl PrettyPrint for GenericArgumentNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        if !self.terms.is_empty() {
            terms.push(allocator.text("⦓"));
            terms.push(allocator.intersperse(
                self.terms.iter().map(|x| {
                    let mut terms = Vec::with_capacity(5);
                    terms.push(allocator.generic(x.key.name.clone()));
                    if let Some(k) = &x.value {
                        terms.push(allocator.text(": "));
                        terms.push(k.build(allocator));
                    }
                    if let Some(k) = &x.value {
                        terms.push(allocator.text(" = "));
                        terms.push(k.build(allocator));
                    }
                    allocator.concat(terms)
                }),
                allocator.text(", "),
            ));
            terms.push(allocator.text("⦔"));
        }
        allocator.concat(terms)
    }
}

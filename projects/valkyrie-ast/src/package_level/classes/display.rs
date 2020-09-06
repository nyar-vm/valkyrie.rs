use super::*;

impl PrettyPrint for ClassDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        terms.push(allocator.keyword("class"));
        terms.push(allocator.space());
        terms.push(self.namepath.build(allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for ClassFieldDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        terms.push(self.modifiers.build(allocator));
        terms.push(allocator.argument(self.field_name.name.to_string(), false));
        if let Some(typing) = &self.r#type {
            terms.push(allocator.keyword(":"));
            terms.push(allocator.space());
            terms.push(typing.build(allocator));
        }
        if let Some(value) = &self.default {
            terms.push(allocator.keyword("="));
            terms.push(allocator.space());
            terms.push(value.build(allocator));
        }
        allocator.concat(terms)
    }
}

impl PrettyPrint for ClassMethodDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        // terms.push(self.modifiers.build(allocator));
        terms.push(allocator.argument(self.method_name.name.to_string(), false));
        // terms.push(self.parameters.build(allocator));
        // if let Some(typing) = &self.r#type {
        //     terms.push(allocator.keyword(":"));
        //     terms.push(allocator.space());
        //     terms.push(typing.build(allocator));
        // }
        // terms.push(allocator.keyword("=>"));
        // terms.push(allocator.space());
        // terms.push(self.body.build(allocator));
        allocator.concat(terms)
    }
}

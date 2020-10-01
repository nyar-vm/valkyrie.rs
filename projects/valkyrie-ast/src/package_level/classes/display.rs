use super::*;

impl PrettyPrint for ClassDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms.push(theme.keyword("class"));
        terms.push(theme.space());
        terms.push(self.namepath.build(theme));
        // terms.push(theme.space());
        terms.push(self.body.build(theme));
        theme.concat(terms)
    }
}

impl PrettyPrint for ClassFieldDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms.push(self.modifiers.build(theme));
        terms.push(theme.argument(self.field_name.name.to_string(), false));
        if let Some(typing) = &self.r#type {
            terms.push(theme.keyword(":"));
            terms.push(theme.space());
            terms.push(typing.build(theme));
        }
        if let Some(value) = &self.default {
            terms.push(theme.keyword("="));
            terms.push(theme.space());
            terms.push(value.build(theme));
        }
        theme.concat(terms)
    }
}

impl PrettyPrint for ClassMethodDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        // terms.push(self.modifiers.build(theme));
        terms.push(theme.argument(self.method_name.name.to_string(), false));
        // terms.push(self.parameters.build(theme));
        // if let Some(typing) = &self.r#type {
        //     terms.push(theme.keyword(":"));
        //     terms.push(theme.space());
        //     terms.push(typing.build(theme));
        // }
        // terms.push(theme.keyword("=>"));
        // terms.push(theme.space());
        // terms.push(self.body.build(theme));
        theme.concat(terms)
    }
}

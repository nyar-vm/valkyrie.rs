use super::*;

impl PrettyPrint for ClassDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("class");
        terms += " ";
        terms += self.namepath.pretty(theme);
        // terms += " ";
        terms += self.body.pretty(theme);
        terms.into()
    }
}

impl PrettyPrint for ClassFieldDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += self.modifiers.pretty(theme);
        terms += theme.argument(self.field_name.name.to_string(), false);
        if let Some(typing) = &self.r#type {
            terms += theme.keyword(":");
            terms += " ";
            terms += typing.pretty(theme);
        }
        if let Some(value) = &self.default {
            terms += theme.keyword("=");
            terms += " ";
            terms += value.pretty(theme);
        }
        terms.into()
    }
}

impl PrettyPrint for ClassMethodDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += self.modifiers.pretty(theme);
        terms += theme.operator(self.method_name.name.to_string());
        if let Some(typing) = &self.generic {
            if !typing.terms.is_empty() {
                terms += typing.pretty(theme);
            }
        }
        terms += self.arguments.pretty(theme);
        if let Some(typing) = &self.return_type {
            terms += typing.pretty(theme);
        }
        if let Some(value) = &self.effect_type {
            terms += value.pretty(theme);
        }
        if let Some(value) = &self.body {
            terms += value.pretty(theme);
        }
        terms.into()
    }
}

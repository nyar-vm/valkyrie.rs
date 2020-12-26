use super::*;
use pretty_print::helpers::SoftBlock;

impl PrettyPrint for ClassDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("class");
        terms += " ";
        terms += self.identifier.pretty(theme);
        if let Some(gen) = &self.generic {
            terms += gen.pretty(theme);
        }
        terms += " ";
        let block = SoftBlock::curly_braces().with_joint(PrettyTree::text(";").append(PrettyTree::Hardline));
        terms += block.join_slice(&self.body.terms, theme);
        terms.into()
    }
}

impl Lispify for ClassDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("define/class");
        lisp += self.identifier.lispify();
        lisp += self.modifiers.lispify();
        for item in &self.body.terms {
            lisp += item.lispify();
        }
        lisp
    }
}

impl PrettyPrint for ClassFieldDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += self.modifiers.pretty(theme);
        terms += theme.argument(self.field_name.name.to_string(), false);
        if let Some(typing) = &self.r#type {
            terms += theme.operator(":");
            terms += " ";
            terms += typing.pretty(theme);
        }
        if let Some(value) = &self.default {
            terms += theme.operator("=");
            terms += " ";
            terms += value.pretty(theme);
        }
        terms.into()
    }
}

impl PrettyPrint for ClassMethodDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
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
            terms += " ";
            terms += value.pretty(theme);
        }
        if let Some(value) = &self.body {
            terms += " ";
            terms += value.pretty(theme);
        }
        terms.into()
    }
}

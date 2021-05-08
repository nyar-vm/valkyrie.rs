use super::*;

impl Debug for ClassTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Field(v) => Debug::fmt(v, f),
            Self::Method(v) => Debug::fmt(v, f),
            Self::Domain(v) => Debug::fmt(v, f),
        }
    }
}
impl ClassKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Class => "class",
            Self::Structure => "structure",
        }
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for ClassDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("class");
        terms += " ";
        terms += self.name.pretty(theme);
        if let Some(gen) = &self.generic {
            terms += gen.pretty(theme);
        }
        terms += " ";
        let block = SoftBlock::curly_braces().with_joint(PrettyTree::text(";").append(PrettyTree::Hardline));
        terms += block.join_slice(&self.body, theme);
        terms += block.join_slice(&self.methods, theme);
        terms.into()
    }
}
#[cfg(feature = "lispify")]
impl Lispify for ClassDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("define/class");
        lisp += self.name.lispify();
        lisp += self.modifiers.lispify();
        for item in &self.body {
            lisp += item.lispify();
        }
        for item in &self.methods {
            lisp += item.lispify();
        }
        lisp
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for FieldDeclaration {
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
#[cfg(feature = "lispify")]
impl Lispify for FieldDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(10);
        lisp += Lisp::keyword("class/field");
        lisp += self.field_name.lispify();
        lisp += self.modifiers.lispify();
        if let Some(typing) = &self.r#type {
            lisp += Lisp::keyword(":");
            lisp += typing.lispify();
        }
        if let Some(value) = &self.default {
            lisp += Lisp::keyword("=");
            lisp += value.lispify();
        }
        lisp
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for MethodDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += self.modifiers.pretty(theme);
        terms += theme.operator(self.method_name.to_string());
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
#[cfg(feature = "lispify")]
impl Lispify for MethodDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("class/method");
        lisp += self.method_name.lispify();
        lisp += self.modifiers.lispify();
        if let Some(generic) = &self.generic {
            if !generic.terms.is_empty() {
                lisp += generic.lispify();
            }
        }
        lisp += self.arguments.lispify();
        match &self.return_type {
            Some(ty) => {
                lisp += Lisp::keyword("return/type") + ty.lispify();
            }
            None => {
                lisp += Lisp::keyword("return/type") + Lisp::symbol("Unit");
            }
        }
        match &self.effect_type {
            Some(ty) => lisp += ty.lispify(),
            None => {
                lisp += Lisp::keyword("return/type") + Lisp::symbol("Pure");
            }
        }
        if let Some(body) = &self.body {
            for item in &body.terms {
                lisp += item.lispify();
            }
        }
        lisp
    }
}

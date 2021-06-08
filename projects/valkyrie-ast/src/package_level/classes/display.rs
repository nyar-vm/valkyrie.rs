use super::*;

impl Debug for ClassTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Macro(v) => Debug::fmt(v, f),
            Self::Field(v) => Debug::fmt(v, f),
            Self::Method(v) => Debug::fmt(v, f),
            Self::Domain(v) => Debug::fmt(v, f),
        }
    }
}
impl ClassKind {
    /// The keyword of class declaration
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
        terms += block.join_slice(&self.terms, theme);
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
        for item in &self.terms {
            lisp += item.lispify();
        }
        for item in &self.methods {
            lisp += item.lispify();
        }
        lisp
    }
}

impl Debug for FieldDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let w = &mut f.debug_struct("Field");
        if !self.annotations.is_empty() {
            w.field("annotations", &self.annotations);
        }
        w.field("name", &WrapDisplay::new(&self.name));
        if let Some(typing) = &self.typing {
            w.field("type", typing);
        }
        if let Some(default) = &self.default {
            w.field("default", default);
        }
        w.finish()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for FieldDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += self.annotations.pretty(theme);
        terms += theme.argument(self.name.name.to_string(), false);
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
        lisp += self.name.lispify();
        lisp += self.annotations.lispify();
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
        terms += self.annotations.pretty(theme);
        terms += theme.operator(self.name.to_string());
        if let Some(typing) = &self.generics {
            if !typing.terms.is_empty() {
                terms += typing.pretty(theme);
            }
        }
        terms += self.parameters.pretty(theme);
        if let Some(typing) = &self.returns {
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
        lisp += self.name.lispify();
        lisp += self.annotations.lispify();
        if let Some(generic) = &self.generics {
            if !generic.terms.is_empty() {
                lisp += generic.lispify();
            }
        }
        lisp += self.parameters.lispify();
        match &self.returns {
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

use super::*;

impl FunctionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            FunctionKind::Macro => "macro",
            FunctionKind::Micro => "micro",
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for FunctionKind {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword(self.as_str())
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for FunctionDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        for m in &self.modifiers {
            terms += theme.keyword(m.name.clone());
            terms += " ";
        }
        terms += theme.keyword(self.r#type.as_str());
        terms += " ";
        terms += self.name.pretty(theme);
        if let Some(gen) = &self.generic {
            terms += gen.pretty(theme);
        }
        terms += self.parameters.pretty(theme);
        if let Some(ret) = &self.r#return {
            terms += ": ";
            terms += ret.typing.pretty(theme);
        }
        terms += self.body.pretty(theme);
        terms.into()
    }
}
#[cfg(feature = "lispify")]
impl Lispify for FunctionDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(6);
        // lisp += self.r#type.lispify();
        lisp += self.name.lispify();
        if let Some(generic) = &self.generic {
            lisp += generic.lispify();
        }
        lisp += self.parameters.lispify();
        if let Some(r#return) = &self.r#return {
            lisp += r#return.lispify();
        }
        // lisp += self.body.lispify();
        lisp
    }
}

impl Debug for FunctionReturnNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if self.is_empty() {
            f.debug_struct("Auto").finish()
        }
        else {
            let w = &mut f.debug_struct("ReturnType");
            if let Some(s) = &self.typing {
                w.field("main", &s);
            }
            if !self.effect.is_empty() {
                w.field("effects", &self.effect);
            }
            w.finish()
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for FunctionReturnNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.operator(":");
        terms += " ";
        terms += self.typing.pretty(theme);
        terms.into()
    }
}
#[cfg(feature = "lispify")]
impl Lispify for FunctionReturnNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for FunctionEffectNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.operator("/");
        terms += " ";
        terms += SoftBlock::brackets().join_slice(&self.effects, theme);
        terms.into()
    }
}
#[cfg(feature = "lispify")]
impl Lispify for FunctionEffectNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementBlock {
    /// ```vk
    /// # inline style
    /// { ... }
    ///
    /// # block style
    /// {
    ///    ...
    /// }
    /// ```
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let block = SoftBlock::curly_braces();
        block.join_slice(&self.terms, theme)
    }
}
#[cfg(feature = "pretty-print")]
impl<K: PrettyPrint, V: PrettyPrint, D: PrettyPrint> PrettyPrint for ArgumentTermNode<K, V, D> {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += self.key.pretty(theme);
        if let Some(value) = &self.value {
            terms += ": ";
            terms += value.pretty(theme);
        }
        if let Some(default) = &self.default {
            terms += " = ";
            terms += default.pretty(theme);
        }
        terms.into()
    }
}

use super::*;

// noinspection DuplicatedCode
#[cfg(feature = "pretty-print")]
impl PrettyPrint for GenericCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += "⦓";
        terms += theme.join(self.terms.clone(), ", ");
        terms += "⦔";
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for GenericCallTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        if let Some(k) = &self.term.key {
            terms += theme.generic(k.name.to_owned());
            terms += ": ";
        }
        terms += self.term.value.pretty(theme);
        terms.into()
    }
}

impl Debug for ParametersList {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.terms.iter()).finish()
    }
}

// noinspection DuplicatedCode
#[cfg(feature = "pretty-print")]
impl PrettyPrint for ParametersList {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += "⦓";
        terms += theme.join(self.terms.clone(), ", ");
        terms += "⦔";
        terms.into()
    }
}
#[cfg(feature = "lispify")]
impl Lispify for ParametersList {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
impl Debug for ParameterTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LMark => f.write_str("<<<disable-index-parameters>>>"),
            Self::RMark => f.write_str("<<<require-named-parameters>>>"),
            Self::Single { annotations, key, bound, default } => {
                let w = &mut f.debug_struct("Parameter");
                w.field("key", &key.name);
                if !annotations.is_empty() {
                    w.field("annotations", annotations);
                }
                if let Some(bound) = bound {
                    w.field("bound", bound);
                }
                if let Some(default) = default {
                    w.field("default", default);
                }
                w.finish()
            }
            Self::UnpackList { modifiers, key, bound } => {
                let w = &mut f.debug_struct("UnpackList");
                w.field("key", &key.name);
                if !modifiers.is_empty() {
                    w.field("modifiers", modifiers);
                }
                if let Some(bound) = bound {
                    w.field("bound", bound);
                }
                w.finish()
            }
            Self::UnpackDict { modifiers, key, bound } => {
                let w = &mut f.debug_struct("UnpackDict");
                w.field("key", &key.name);
                if !modifiers.is_empty() {
                    w.field("modifiers", modifiers);
                }
                if let Some(bound) = bound {
                    w.field("bound", bound);
                }
                w.finish()
            }
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ParameterTerm {
    fn pretty(&self, _: &PrettyProvider) -> PrettyTree {
        todo!()
    }
}

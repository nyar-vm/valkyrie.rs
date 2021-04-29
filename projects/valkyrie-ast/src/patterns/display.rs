use super::*;
#[cfg(feature = "pretty-print")]
impl PrettyPrint for PatternBlock {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += "{";
        terms += PrettyTree::Hardline;
        let mut inner = PrettySequence::new(10);
        let len = self.branches.len();
        for (idx, branch) in self.branches.iter().enumerate() {
            inner += branch.pretty(theme);
            if idx == len.saturating_sub(1) {
            }
            else {
                inner += PrettyTree::Hardline;
            }
        }
        terms += inner.indent(4);
        terms += PrettyTree::Hardline;
        terms += "}";
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for PatternBranch {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.concat(vec![self.condition.pretty(theme), self.statements.pretty(theme)])
    }
}
#[cfg(feature = "pretty-print")]

impl PrettyPrint for PatternCondition {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let item = match self {
            Self::Case(v) => v.pretty(theme),
            Self::When(v) => v.pretty(theme),
            Self::Type(v) => v.pretty(theme),
            Self::Else(v) => v.pretty(theme),
        };
        item.append(":").append(PrettyTree::Hardline)
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for PatternStatements {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(self.terms.len() * 2);
        let len = self.terms.len();
        for (idx, term) in self.terms.iter().enumerate() {
            terms += term.pretty(theme);
            if idx == len.saturating_sub(1) {
                terms += ",";
            }
        }
        terms.indent(4)
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for PatternNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Symbol(v) => v.pretty(theme),
            Self::Tuple(v) => v.pretty(theme),
            Self::Class(v) => v.pretty(theme),
            Self::Array(v) => v.pretty(theme),
            Self::Union(v) => v.pretty(theme),
            Self::Atom(v) => v.pretty(theme),
        }
    }
}
#[cfg(feature = "pretty-print")]

impl PrettyPrint for TuplePatternNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        if let Some(bind) = &self.bind {
            terms += bind.pretty(theme);
            terms += "<-";
        }
        if let Some(name) = &self.name {
            terms += name.pretty(theme);
        }
        let block = SoftBlock::parentheses().with_joint(PrettyTree::line_or_comma());
        terms += block.join_slice(&self.terms, theme);
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]

impl PrettyPrint for ClassPatternNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        if let Some(bind) = &self.bind {
            terms += bind.pretty(theme);
            terms += "<-";
        }
        if let Some(name) = &self.name {
            terms += name.pretty(theme);
        }
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for ArrayPatternNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        if let Some(bind) = &self.bind {
            terms += bind.pretty(theme);
            terms += "<-";
        }
        // if let Some(name) = &self.name {
        //     terms += name.pretty(theme);
        // }
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for UnionPatternNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        if let Some(bind) = &self.bind {
            terms += bind.pretty(theme);
            terms += "<-";
        }
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImplicitCaseNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += self.pattern.pretty(theme);
        terms += theme.keyword("≔");
        terms += self.body.pretty(theme);
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for PatternCaseNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(5);
        terms += theme.keyword("case");
        terms += " ";
        terms += self.pattern.pretty(theme);
        if let Some(guard) = &self.guard {
            terms += theme.keyword("when");
            terms += guard.pretty(theme);
        }
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for IdentifierPattern {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += self.modifiers.pretty(theme);
        terms += self.identifier.pretty(theme);
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for PatternWhenNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.concat(vec![theme.keyword("when"), " ".into(), self.guard.pretty(theme)])
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for PatternTypeNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.concat(vec![theme.keyword("type"), " ".into(), self.pattern.pretty(theme)])
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for PatternElseNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword("else")
    }
}

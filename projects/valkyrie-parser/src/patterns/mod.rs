use crate::{helpers::ProgramState, ProgramContext};
use nyar_error::{Success, Validation};
use valkyrie_ast::*;

impl crate::LetPatternNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<PatternNode> {
        match self {
            Self::BarePattern(v) => v.build(ctx),
            Self::StandardPattern(v) => v.build(ctx),
        }
    }
}
impl crate::StandardPatternNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<PatternNode> {
        match self {
            Self::TuplePattern(v) => v.build(ctx),
        }
    }
}

impl crate::BarePatternNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<PatternNode> {
        let mut errors = vec![];
        let mut terms = vec![];
        for node in &self.bare_pattern_item {
            node.build(ctx).append(&mut terms, &mut errors)
        }
        let tuple = TuplePatternNode { bind: None, name: None, terms, span: Default::default() };
        Success { value: PatternNode::Tuple(Box::new(tuple)), diagnostics: errors }
    }
}

impl crate::BarePatternItemNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<PatternNode> {
        let mut errors = vec![];
        let identifier = self.identifier.build(ctx);
        let id = IdentifierPattern { modifiers: Default::default(), identifier };
        Success { value: PatternNode::Atom(Box::new(id)), diagnostics: errors }
    }
}

impl crate::TuplePatternNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<PatternNode> {
        let mut errors = vec![];
        let mut terms = vec![];
        for node in &self.pattern_item {
            node.build(ctx).append(&mut terms, &mut errors)
        }
        let tuple = TuplePatternNode { bind: None, name: None, terms, span: Default::default() };
        Success { value: PatternNode::Tuple(Box::new(tuple)), diagnostics: errors }
    }
}
impl crate::PatternItemNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<PatternNode> {
        let value = match self {
            Self::OmitDict => PatternNode::Atom(Box::new(IdentifierPattern {
                modifiers: Default::default(),
                identifier: IdentifierNode { name: "".to_string(), span: Default::default() },
            })),
            Self::OmitList => PatternNode::Atom(Box::new(IdentifierPattern {
                modifiers: Default::default(),
                identifier: IdentifierNode { name: "".to_string(), span: Default::default() },
            })),
            Self::TuplePatternItem(v) => v.build(ctx)?,
        };
        Success { value, diagnostics: vec![] }
    }
}

impl crate::TuplePatternItemNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<PatternNode> {
        let mut errors = vec![];
        let identifier = self.identifier.build(ctx);
        let id = IdentifierPattern { modifiers: Default::default(), identifier };
        Success { value: PatternNode::Atom(Box::new(id)), diagnostics: errors }
    }
}

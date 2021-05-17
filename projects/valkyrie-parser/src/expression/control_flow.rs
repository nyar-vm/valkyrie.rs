use super::*;

// r#"^(?x)(
//       [.]{2}[=<]
//     | [‥…]
//     # | [.]{1,3}
//     # | [⟦⟧⟬⟭{}\[\]()]
//     | [$§¶^]
//     | @[*!?@]?
//     # contains
//     | (\b(not)\b\s+)?\b(in)\b
//     | [∈∊∉]
//     # is
//     | \b(is)\b(\s+\b(not)\b)?
//     | [⊑⋢]
//     # start with <, >
//     | === | ≡
//     | !== | =!= | ≢
//     | == | ≖
//     | != | ≠
//     | <=>
//     # start with <, >
//     | >{1,3} | >= | /> | ≥ | ⩾ | ≫ | ⋙
//     | <{1,3} | <= | </ | ≤ | ⩽ | ≪ | ⋘ |  <: | <!
//     # start with :
//     | :>
//     # start with -
//     | -> | ⟶ | -{1,2}=?
//     # start with ~
//     | ~> | ~
//     # start with +
//     | [+]> | [+]{1,2}=?
//     # start with *
//     | [*]=?
//     # start with / or % or ÷
//     | /=?
//     | ÷=?
//     | %=?
//     # start with &
//     | &> | &{1,2} | ≻
//     | [|]> | [|]{1,2} | ⊁
//     | ⊻=? | ⊼=? | ⊽=? | [⩕⩖]
//     # start with =
//     | => | ⇒
//     | =
//     # unicode
//     | [⨳∀∁∂∃∄¬±√∛∜⊹⋗]
//     | [↻↺⇆↹⇄⇋⇌⇅]
// )"#

impl crate::ControlFlowNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ControlNode> {
        let expression = match &self.main_expression {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        Success {
            value: ControlNode { kind: self.kw_control.build(ctx), label: "".to_string(), expression, span: self.span.clone() },
            diagnostics: vec![],
        }
    }
}

impl crate::KwControlNode {
    pub fn build(&self, _: &mut ProgramState) -> ControlKind {
        match self {
            Self::Break => ControlKind::Break,
            Self::Continue => ControlKind::Continue,
            Self::Fallthrough => ControlKind::Fallthrough,
            Self::Raise => ControlKind::Resume,
            Self::Resume => ControlKind::Continue,
            Self::Return => ControlKind::Continue,
            Self::YieldBreak => ControlKind::Continue,
            Self::YieldFrom => ControlKind::Continue,
            Self::YieldReturn => ControlKind::Continue,
            Self::YieldSend => ControlKind::Continue,
        }
    }
}

use super::*;

static BINARY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)
      \#
    | [.]{2}[=<]
    | [.]{1,3}
    | [{}\[\]()]
    | [,;$§¶^]
    | @[*!?@]?
    | [!]?
    # start with <, >
    | >{1,3} | >= | /> | ≥ | ⩾ | ≫
    | <{1,3} | <= | </ | ≤ | ⩽ | <: | <! 
    # start with :
    | ∷ | :: | :> | := | ≔ | :
    # start with -
    | -= | -> | ⟶ | -{1,2}
    # start with ~
    | ~> | ~
    # start with +
    | [+]= | [+]> | [+]{1,2}
    # start with *
    | [*]=?
    # start with / or % or ÷
    | /=?
    | ÷=?
    | %=?
    # start with &
    | &> | &{1,2} | ≻
    | [|]> | [|]{1,2} | ⊁
    | ⊻=? | ⊼=? | ⊽=? | [⩕⩖]
    # start with !
    | != | ≠ | !
    # start with ?
    | [?]{3} | [?]
    # start with =
    | => | ⇒
    | === | == | =
    # unicode
    | [∈∊∉⊑⋢⨳∀∁∂∃∄¬±√∛∜⊹⋗]
    | [⟦⟧⁅⁆⟬⟭]
    | [↻↺⇆↹⇄⇋⇌⇅]
"#,
    )
    .unwrap()
});

impl ValkyrieInfix {
    pub fn parse(input: &str) -> Option<Self> {
        let mut range = Range { start: 0, end: 0 };
        let mut normalized = input;
        if let Some(caps) = BINARY.lock().captures(input) {
            range.end = caps.get(0).unwrap().end();
            normalized = caps.get(0).unwrap().as_str();
        }
        Some(Self { normalized, range })
    }
}

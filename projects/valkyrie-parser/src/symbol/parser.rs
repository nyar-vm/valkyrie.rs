use super::*;

impl FromStr for ValkyrieIdentifier {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

pub static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ux)
    ^(?P<regular>(?:\p{XID_Start}|_)\p{XID_Continue}*)
|   ^`(?P<escaped>(?:\\.|[^`])*)`
    ",
    )
    .unwrap()
});

impl ValkyrieIdentifier {
    /// - regular: `\p{XID_Start}|_)\p{XID_Continue}*`
    /// - escaped: ``` `(\.|[^`])*` ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&IDENTIFIER, "IDENTIFIER")?;
        let id = ValkyrieIdentifier { name: m.as_str().to_string(), range: state.away_from(input) };
        state.finish(id)
    }
}

impl ValkyrieNamepath {
    /// `id (~ :: ~ b)*`
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let mut names = Vec::new();
        let (state, id) = input.match_fn(ValkyrieIdentifier::parse)?;
        names.push(id);
        let (state, _) = state.match_repeats(|s| pare_colon_id(s, &mut names))?;
        state.finish(ValkyrieNamepath { names, range: state.away_from(input) })
    }
}

fn pare_colon_id<'i>(input: ParseState<'i>, names: &mut Vec<ValkyrieIdentifier>) -> ParseResult<'i, ()> {
    let (state, _) = input
        .begin_choice()
        .or_else(|s| s.match_str("::").map_inner(|_| ()))
        .or_else(|s| s.match_char('∷').map_inner(|_| ()))
        .end_choice()?;
    let (state, id) = state.match_fn(|s| ValkyrieIdentifier::parse(s))?;
    names.push(id);
    state.finish(())
}

#[test]
pub fn test() {
    let input = "`_he\\u2222llo`";
    let id = ValkyrieIdentifier::from_str(input).unwrap();
    println!("{}", id)
}

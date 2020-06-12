// use crate::LexerContext;
// use nyar_hir::{ASTKind, Result};
// use std::{fmt::Write as _, fs::File, io::Write};
//
// pub trait ASTDump {
//     fn parse(input: &str) -> Self;
//     fn save(&self, path: &str) -> Result<()>;
// }
//
// impl ASTDump for ASTKind {
//     fn parse(input: &str) -> Self {
//         let mut cfg = LexerContext::default();
//         cfg.refine = true;
//         cfg.get_ast(input).unwrap()
//     }
//     fn save(&self, path: &str) -> Result<()> {
//         let mut out = String::new();
//         match self {
//             ASTKind::Program(v) => {
//                 for i in v {
//                     writeln!(out, "{:#?}", i)?;
//                 }
//             }
//             _ => unreachable!(),
//         }
//         let mut file = File::create(path)?;
//         file.write_all(out.as_bytes())?;
//         Ok(())
//     }
// }

use crate::helpers::ignore;
use lispify::Lisp;
use valkyrie_types::third_party::pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};

pub trait ThisParser
where
    Self: Sized,
{
    fn parse(input: ParseState) -> ParseResult<Self>;
    fn parse_text(input: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(input.trim_end());
        make_from_str(state.skip(whitespace), Self::parse)
    }
    fn parse_many(input: ParseState) -> ParseResult<Vec<Self>> {
        let (state, terms) = input.match_repeats(Self::parse)?;
        if state.skip(ignore).residual.is_empty() { state.finish(terms) } else { StopBecause::expect_eof(state.start_offset)? }
    }
    fn as_lisp(&self) -> Lisp;
}

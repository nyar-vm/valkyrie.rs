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
use valkyrie_types::{
    third_party::pex::{
        helpers::{make_from_str, whitespace},
        ParseResult, ParseState, StopBecause,
    },
    ValkyrieError, ValkyrieResult,
};

pub trait ThisParser
where
    Self: Sized,
{
    fn parse(input: ParseState) -> ParseResult<Self>;
    fn parse_text(input: &str) -> ValkyrieResult<Self> {
        let input = ParseState::new(input);
        let (state, repl) = match Self::parse(input.skip(ignore)) {
            ParseResult::Pending(s, v) => (s.skip(ignore), v),
            ParseResult::Stop(e) => Err(ValkyrieError::custom(format!("Failed to parse REPL: {:?}", e)))?,
        };
        if !state.residual.is_empty() {
            Err(ValkyrieError::custom(format!("Expect EOF, found:\n{}", state.residual)))?
        }
        Ok(repl)
    }
    fn as_lisp(&self) -> Lisp;
}

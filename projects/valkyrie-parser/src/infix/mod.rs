use crate::expression::{ValkyrieOperator, ValkyrieOperatorKind};
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};
use pratt::Precedence;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};

use pratt::{Associativity, Precedence};
use regex::Regex;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    str::FromStr,
    sync::LazyLock,
};
mod display;
mod parser;
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};
use regex::Regex;
use std::{str::FromStr, sync::LazyLock};

mod display;
mod parser;

#[derive(Clone, Debug, Eq)]
pub struct ValkyrieOperator {
    pub kind: ValkyrieOperatorKind,
    pub range: Range<usize>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ValkyrieOperatorKind {
    /// prefix operator: `!`
    Not,
    /// prefix operator: `+`
    Positive,
    /// prefix operator: `-`
    Negative,
    /// prefix operator: `*`
    Unbox,
    /// prefix operator: `**`
    Unpack,
    /// prefix operator: `⅟`
    Inverse,
    /// prefix operator: `⅟`
    Surd(u8),
    /// binary operator: `+`
    Plus,
    /// binary operator: `++`
    Concat,
    /// binary operator: `>`
    Greater,
    /// binary operator: `<`
    Less,
    /// binary operator: `≫`, `>>`
    MuchGreater,
    /// binary operator:
    MuchLess,
    /// binary operator: `⋙`, `>>>`
    VeryMuchGreater,
    /// binary operator:
    VeryMuchLess,
    /// binary operator: `≡`, `≢`
    Equal(bool),
    /// binary operator:
    StrictlyEqual(bool),
    /// binary operator:
    Belongs(bool),
    /// binary operator: `-`
    Minus,
    /// binary operator: `*`
    Multiply,
    /// binary operator: `/`
    Divide,
    /// binary operator: `^`
    Power,
    /// suffix operator: `!`
    Unwrap,
    /// suffix operator: `?`
    Raise,
    /// suffix operator: `℃`
    Celsius,
    /// suffix operator: `℉`
    Fahrenheit,
    /// suffix operator: `a%, b‰, c‱`
    DivideByDecimalPower(u8),
    /// suffix operator: `ᵀ`, `\^T`, `\transpose`
    Transpose,
    /// suffix operator: `ᴴ`, `\^H`, `\conjugate_transpose
    Transjugate,
    Hermitian,
}

#[derive(Clone)]
pub struct ValkyrieInfix {
    normalized: String,
    range: Range<usize>,
}

#[derive(Clone)]
pub struct ValkyriePrefix {
    normalized: String,
    range: Range<usize>,
}

#[derive(Clone)]
pub struct ValkyrieSuffix {
    normalized: String,
    range: Range<usize>,
}

use crate::SyntaxError;
use pex::StopBecause;
use std::ops::Range;

impl From<StopBecause> for SyntaxError {
    fn from(error: StopBecause) -> Self {
        let Range { start, end } = error.range();
        SyntaxError {
            info: error.to_string(),
            span: Default::default(),
            span: (start as u32)..(end as u32),
            level: Default::default(),
        }
    }
}

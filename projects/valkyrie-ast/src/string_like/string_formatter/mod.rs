use super::*;

mod display;

use crate::{ArgumentTerm, ExpressionNode, StringTextNode};

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringFormatted {
    /// The raw string of the number.
    pub terms: Vec<StringFormattedTerm>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `\r, \u{00FF}, {{, }}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringFormattedTerm {
    /// `abc \r\u{12}`
    Text {
        /// The unescaped string of the number.
        unescaped: StringTextNode,
    },
    /// `{ datetime, 'yyyy-MM-dd h:mm tt', culture: 'en-us' }`
    ///
    /// equivalent to
    ///
    /// `{ datetime.show(fmt: 'yyyy-MM-dd h:mm tt', culture: 'en-us')}`
    Simple {
        /// The raw string of the number.
        argument: ExpressionKind,
        /// The format arguments
        formatter: Option<StringTextNode>,
    },
    /// `{ datetime, fmt:yyyy-MM-dd h:mm tt, culture:en-us }`
    ///
    /// equivalent to
    ///
    /// `{ datetime.show(fmt: 'yyyy-MM-dd h:mm tt', culture: 'en-us')}`
    Complex {
        /// The raw string of the number.
        argument: ExpressionKind,
        /// The format arguments
        formatters: Vec<ArgumentTerm>,
    },
}

impl ValkyrieNode for StringFormatted {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}

impl StringFormatted {
    /// Create a string formatter
    pub fn new(capacity: usize, span: &Range<u32>) -> Self {
        Self { terms: Vec::with_capacity(capacity), span: span.clone() }
    }
}

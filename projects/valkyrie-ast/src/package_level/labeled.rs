use super::*;

/// `⁜label.context.1`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GotoStatement {
    /// The goto path
    pub path: Vec<String>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `※label.context.1`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LabelStatement {
    /// The label path
    pub path: Vec<String>,
    /// The range of the node
    pub span: Range<u32>,
}

impl Display for GotoStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_char('⁜')?;
        for (i, path) in self.path.iter().enumerate() {
            if i != 0 {
                f.write_char('.')?;
            }
            f.write_str(path)?;
        }
        Ok(())
    }
}

impl Display for LabelStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_char('※')?;
        for (i, path) in self.path.iter().enumerate() {
            if i != 0 {
                f.write_char('.')?;
            }
            f.write_str(path)?;
        }
        Ok(())
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for LabelStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.annotation(self.to_string())
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for GotoStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.annotation(self.to_string())
    }
}

use super::*;
#[cfg(feature = "pex")]
use pex::StopBecause;

#[cfg(feature = "pex")]
impl From<StopBecause> for ValkyrieError {
    fn from(error: StopBecause) -> Self {
        ValkyrieError::syntax_error(error.to_string(), error.range())
    }
}

use crate::RuntimeError;
use dashu::base::ConversionError;
use diagnostic::DiagnosticLevel;

impl From<ConversionError> for RuntimeError {
    fn from(value: ConversionError) -> Self {
        Self { info: value.to_string(), level: DiagnosticLevel::Error }
    }
}

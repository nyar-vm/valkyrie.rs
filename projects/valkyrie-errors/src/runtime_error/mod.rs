use diagnostic::DiagnosticLevel;

#[cfg(feature = "dashu")]
mod for_dashu;

#[derive(Clone, Debug)]
pub struct RuntimeError {
    pub info: String,
    pub level: DiagnosticLevel,
}

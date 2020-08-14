use diagnostic::DiagnosticLevel;

#[derive(Clone, Debug)]
pub struct RuntimeError {
    pub info: String,
    pub level: DiagnosticLevel,
}

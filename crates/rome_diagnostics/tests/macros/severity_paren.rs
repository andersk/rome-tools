use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(severity(Error))]
struct TestDiagnostic {}

fn main() {}

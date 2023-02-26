use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(message(unknown))]
struct TestDiagnostic {}

fn main() {}

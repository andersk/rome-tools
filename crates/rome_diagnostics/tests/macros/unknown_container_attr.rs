use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(unknown_attr)]
struct TestDiagnostic {}

fn main() {}

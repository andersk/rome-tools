use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(message(description = "text {unclosed"))]
struct TestDiagnostic {}

fn main() {}

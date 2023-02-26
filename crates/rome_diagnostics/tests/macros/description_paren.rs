use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(message(description("description")))]
struct TestDiagnostic {}

fn main() {}

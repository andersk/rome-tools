use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(category = Identifier)]
struct TestDiagnostic {}

fn main() {}

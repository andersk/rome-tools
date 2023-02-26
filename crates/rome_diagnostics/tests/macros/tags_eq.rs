use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(tags = Identifier)]
struct TestDiagnostic {}

fn main() {}

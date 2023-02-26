use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(tags(Identifier, Identifier))]
struct TestDiagnostic {}

fn main() {}

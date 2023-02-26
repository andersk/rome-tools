use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
#[diagnostic(message(description = Ident))]
struct TestDiagnostic {}

fn main() {}

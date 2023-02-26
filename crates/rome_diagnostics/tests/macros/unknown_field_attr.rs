use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
struct TestDiagnostic {
    #[unknown_attr]
    field: bool,
}

fn main() {}

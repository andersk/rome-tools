use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
struct TestDiagnostic {
    #[location = Identifier]
    location: (),
}

fn main() {}

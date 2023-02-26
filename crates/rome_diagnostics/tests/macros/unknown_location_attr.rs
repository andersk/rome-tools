use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
struct TestDiagnostic {
    #[location(unknown)]
    location: (),
}

fn main() {}

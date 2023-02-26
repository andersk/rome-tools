use rome_diagnostics::Diagnostic;

#[derive(Clone, Debug, Diagnostic)]
enum ErrorEnum {
    Int(u32),
    Float(f32),
}

fn main() {}

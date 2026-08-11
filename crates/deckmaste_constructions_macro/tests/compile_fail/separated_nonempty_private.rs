use deckmaste_construction_compiler::runtime::SeparatedNonEmpty;

fn main() {
    let _ = SeparatedNonEmpty::<u8, char> {
        first: 1,
        rest: Vec::new(),
    };
}

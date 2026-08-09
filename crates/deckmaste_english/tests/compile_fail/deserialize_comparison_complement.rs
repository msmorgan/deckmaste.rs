use deckmaste_english::syntax::ComparisonComplement;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<ComparisonComplement>();
}

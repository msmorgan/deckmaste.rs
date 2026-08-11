use deckmaste_english::syntax::ModifierCoordination;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<ModifierCoordination>();
}

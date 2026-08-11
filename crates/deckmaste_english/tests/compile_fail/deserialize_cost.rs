use deckmaste_english::syntax::Cost;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<Cost>();
}

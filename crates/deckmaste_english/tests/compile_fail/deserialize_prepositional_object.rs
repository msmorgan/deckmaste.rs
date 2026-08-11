use deckmaste_english::syntax::PrepositionalObject;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<PrepositionalObject>();
}

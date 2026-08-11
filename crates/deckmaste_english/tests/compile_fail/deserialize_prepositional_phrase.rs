use deckmaste_english::syntax::PrepositionalPhrase;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<PrepositionalPhrase>();
}

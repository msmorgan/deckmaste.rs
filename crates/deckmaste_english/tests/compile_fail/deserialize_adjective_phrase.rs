use deckmaste_english::syntax::AdjectivePhrase;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<AdjectivePhrase>();
}

use deckmaste_english::syntax::AdjectivePhraseCoordination;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<AdjectivePhraseCoordination>();
}

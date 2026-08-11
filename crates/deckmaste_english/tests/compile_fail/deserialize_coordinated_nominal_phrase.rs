use deckmaste_english::syntax::CoordinatedNominalPhrase;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<CoordinatedNominalPhrase>();
}

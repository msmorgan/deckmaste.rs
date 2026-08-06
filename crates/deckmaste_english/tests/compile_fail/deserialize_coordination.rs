use deckmaste_english::syntax::CoordinatedNominalPhrase;
use deckmaste_english::syntax::CoordinatedNounPhrase;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<CoordinatedNounPhrase>();
    requires_deserialize::<CoordinatedNominalPhrase>();
}

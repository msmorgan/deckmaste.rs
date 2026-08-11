use deckmaste_english::syntax::NominalPhrase;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<NominalPhrase>();
}

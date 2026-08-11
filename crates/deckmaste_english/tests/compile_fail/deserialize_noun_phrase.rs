use deckmaste_english::syntax::NounPhrase;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<NounPhrase>();
}

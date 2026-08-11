use deckmaste_english::syntax::Determiner;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<Determiner>();
}

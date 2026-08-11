use deckmaste_english::syntax::CoordinatedModifier;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<CoordinatedModifier>();
}

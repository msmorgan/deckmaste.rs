use deckmaste_english::syntax::Ability;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<Ability>();
}

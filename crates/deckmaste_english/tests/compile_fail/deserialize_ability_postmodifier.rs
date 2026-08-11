use deckmaste_english::syntax::AbilityPostmodifier;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<AbilityPostmodifier>();
}

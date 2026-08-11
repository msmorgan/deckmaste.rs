use deckmaste_english::syntax::AttachedPredicate;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<AttachedPredicate>();
}

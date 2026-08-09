use deckmaste_english::syntax::PredicateHead;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<PredicateHead>();
}

use deckmaste_english::syntax::InfinitiveClause;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<InfinitiveClause>();
}

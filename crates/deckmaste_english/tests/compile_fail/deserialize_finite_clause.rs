use deckmaste_english::syntax::FiniteClause;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<FiniteClause>();
}

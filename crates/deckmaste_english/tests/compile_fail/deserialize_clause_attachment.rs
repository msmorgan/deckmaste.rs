use deckmaste_english::syntax::ComplexClause;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<ComplexClause>();
}

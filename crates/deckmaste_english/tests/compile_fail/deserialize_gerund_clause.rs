use deckmaste_english::syntax::GerundClause;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<GerundClause>();
}

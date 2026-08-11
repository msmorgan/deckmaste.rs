use deckmaste_english::syntax::ExistentialClause;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<ExistentialClause>();
}

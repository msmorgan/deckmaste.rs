use deckmaste_english::syntax::Determiner;
use deckmaste_english::syntax::Possessor;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<Determiner>();
    requires_deserialize::<Possessor>();
}

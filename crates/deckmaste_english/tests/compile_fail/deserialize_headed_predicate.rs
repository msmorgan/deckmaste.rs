use deckmaste_english::syntax::HeadedPredicate;
use deckmaste_english::syntax::Intransitive;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<HeadedPredicate<Intransitive>>();
}

use deckmaste_english::syntax::Sentence;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<Sentence>();
}

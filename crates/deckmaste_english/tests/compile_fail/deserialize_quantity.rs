use deckmaste_english::syntax::Quantity;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<Quantity>();
}

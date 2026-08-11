use deckmaste_english::syntax::Coordination;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<Coordination<u8>>();
}

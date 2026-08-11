use deckmaste_english::syntax::ExceptionRider;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<ExceptionRider>();
}

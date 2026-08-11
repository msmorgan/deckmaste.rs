use deckmaste_english::word::NounInstance;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<NounInstance>();
}

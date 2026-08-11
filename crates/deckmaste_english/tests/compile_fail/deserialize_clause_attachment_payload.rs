use deckmaste_english::syntax::Attachment;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<Attachment<u8>>();
}

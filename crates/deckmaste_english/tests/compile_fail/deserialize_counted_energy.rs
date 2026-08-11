use deckmaste_english::syntax::CountedEnergy;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<CountedEnergy>();
}

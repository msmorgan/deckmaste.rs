use deckmaste_english_construction_macros::spike_sealed;

spike_sealed!(optout);

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<SpikeOpenRecord>();
}

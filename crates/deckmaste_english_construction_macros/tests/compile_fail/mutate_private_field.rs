use deckmaste_english_construction_macros::spike_sealed;

spike_sealed!(mutation);

fn main() {
    let mut sealed = SpikeCoordination::try_new(2).unwrap();
    sealed.members = 0;
}

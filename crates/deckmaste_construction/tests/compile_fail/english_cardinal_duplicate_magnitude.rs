use deckmaste_construction::constructions;

constructions! {
    codec CardinalNumber {
        generate english_cardinal {
            magnitude = u32;
            magnitude = NonZeroU32;
        }
    }
}

fn main() {}

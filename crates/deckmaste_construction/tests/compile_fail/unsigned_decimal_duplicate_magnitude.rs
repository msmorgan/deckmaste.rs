use deckmaste_construction::constructions;

constructions! {
    codec ScalarNumber {
        generate unsigned_decimal {
            magnitude = u32;
            magnitude = NonZeroU32;
        }
    }
}

fn main() {}

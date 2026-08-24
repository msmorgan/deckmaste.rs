use deckmaste_construction::constructions;

constructions! {
    codec ScalarNumber {
        generate unsigned_decimal {
            magnitude = u64;
        }
    }
}

fn main() {}

use deckmaste_construction::constructions;

constructions! {
    codec SignedNumber {
        generate signed_decimal {
            magnitude = u64;
            sign_type = Sign { Positive = none, Negative = "-", };
        }
    }
}

fn main() {}

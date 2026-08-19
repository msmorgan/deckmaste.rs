use deckmaste_construction::constructions;

constructions! {
    codec SignedNumber {
        generate signed_decimal {
            magnitude = u32;
            sign_type = Sign {
                Positive = none,
                Positive = none,
                Negative = "-",
            };
        }
    }
}

fn main() {}

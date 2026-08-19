use deckmaste_construction::constructions;

constructions! {
    codec SignedNumber {
        value_type = SignedNumber;
        generate signed_decimal {
            magnitude = u32;
            sign_type = Sign { Positive = none, Negative = "-", };
        }
    }
}

fn main() {}

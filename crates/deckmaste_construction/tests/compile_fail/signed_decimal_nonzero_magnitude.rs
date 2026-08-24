use deckmaste_construction::constructions;

constructions! {
    codec SignedNumber {
        generate signed_decimal {
            magnitude = NonZeroU32;
            sign_type = Sign { Positive = none, Negative = "-", };
        }
    }
    construction only: Cat { element Only {} form only = "only"; }
    root Cat { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}

#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
pub struct FixturePhrase;

#[derive(Debug, PartialEq, Eq)]
pub struct AdaptedPhrase {
    phrase: FixturePhrase,
}

deckmaste_constructions_macro::constructions! {
    group fallback_contract;

    construction adapted_phrase: AdaptedPhrase {
        bind AdaptedPhrase {
            phrase: hole FixturePhrase,
        }
        form first @ 0 otherwise = phrase;
        form second @ 1 otherwise = phrase;
    }
}

fn main() {}

#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
pub struct FixturePhrase;

#[derive(Debug, PartialEq, Eq)]
pub struct AdaptedPhrase {
    phrase: FixturePhrase,
}

fn takes_guarded_form(_value: &AdaptedPhrase) -> bool {
    true
}

deckmaste_constructions_macro::constructions! {
    group fallback_contract;

    construction adapted_phrase: AdaptedPhrase {
        bind AdaptedPhrase {
            phrase: hole FixturePhrase,
        }
        form guarded @ 0 when check(takes_guarded_form) otherwise = phrase;
    }
}

fn main() {}

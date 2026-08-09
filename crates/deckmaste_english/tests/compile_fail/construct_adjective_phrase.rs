use deckmaste_english::syntax::AdjectiveComplement;
use deckmaste_english::syntax::AdjectivePhrase;
use deckmaste_english::word::Adjective;

fn bypass(head: Adjective) -> AdjectivePhrase {
    AdjectivePhrase {
        degree: None,
        head,
        complements: Vec::<AdjectiveComplement>::new(),
    }
}

fn main() {}

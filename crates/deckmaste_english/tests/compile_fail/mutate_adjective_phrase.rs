use deckmaste_english::syntax::AdjectivePhrase;
use deckmaste_english::word::Adjective;

fn bypass(mut phrase: AdjectivePhrase, head: Adjective) {
    phrase.head = head;
}

fn main() {}

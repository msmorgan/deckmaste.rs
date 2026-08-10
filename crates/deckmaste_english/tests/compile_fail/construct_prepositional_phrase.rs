use deckmaste_english::syntax::Phrase;
use deckmaste_english::syntax::Preposition;
use deckmaste_english::syntax::PrepositionalPhrase;

fn bypass(object: Phrase) -> PrepositionalPhrase {
    PrepositionalPhrase::simple(Preposition::Of, object)
}

fn main() {}

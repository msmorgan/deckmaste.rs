use deckmaste_english::syntax::CoordinatedNounPhrase;
use deckmaste_english::syntax::NounPhrase;
use deckmaste_english::syntax::NounPhraseCoordination;

fn bypass(first: Box<NounPhrase>, rest: Vec<NounPhraseCoordination>) -> CoordinatedNounPhrase {
    CoordinatedNounPhrase { first, rest }
}

fn main() {}

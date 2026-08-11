use deckmaste_english::syntax::AdjectivePhrase;
use deckmaste_english::syntax::AdjectivePhraseCoordination;
use deckmaste_english::syntax::CoordinatedAdjectivePhrase;

fn bypass_checked_coordination(
    first: Box<AdjectivePhrase>,
    rest: Vec<AdjectivePhraseCoordination>,
) -> CoordinatedAdjectivePhrase {
    CoordinatedAdjectivePhrase { first, rest }
}

fn main() {}

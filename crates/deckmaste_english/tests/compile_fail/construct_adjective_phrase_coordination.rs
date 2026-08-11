use deckmaste_english::features::Conjunction;
use deckmaste_english::syntax::AdjectivePhrase;
use deckmaste_english::syntax::AdjectivePhraseCoordination;

fn bypass_checked_member(
    conjunction: Option<Conjunction>,
    phrase: AdjectivePhrase,
) -> AdjectivePhraseCoordination {
    AdjectivePhraseCoordination {
        conjunction,
        phrase,
    }
}

fn main() {}

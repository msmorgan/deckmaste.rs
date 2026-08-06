use deckmaste_english::syntax::CoordinatedNominalPhrase;
use deckmaste_english::syntax::Determiner;
use deckmaste_english::syntax::NominalComplement;
use deckmaste_english::syntax::NominalPhrase;
use deckmaste_english::syntax::NominalPhraseCoordination;

fn bypass(
    determiner: Determiner,
    first: Box<NominalPhrase>,
    rest: Vec<NominalPhraseCoordination>,
    complements: Vec<NominalComplement>,
) -> CoordinatedNominalPhrase {
    CoordinatedNominalPhrase {
        determiner,
        first,
        rest,
        complements,
    }
}

fn main() {}

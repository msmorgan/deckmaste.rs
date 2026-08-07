use deckmaste_english::syntax::Determiner;
use deckmaste_english::syntax::NominalComplement;
use deckmaste_english::syntax::NominalModifier;
use deckmaste_english::syntax::NominalPhrase;
use deckmaste_english::word::NounInstance;

fn bypass(
    determiner: Option<Determiner>,
    modifiers: Vec<NominalModifier>,
    head: NounInstance,
    complements: Vec<NominalComplement>,
) -> NominalPhrase {
    NominalPhrase {
        determiner,
        modifiers,
        head,
        complements,
    }
}

fn main() {}

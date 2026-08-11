use deckmaste_english::word::Noun;
use deckmaste_english::word::NounInstance;
use deckmaste_english::word::NounInstanceKind;
use deckmaste_english::word::Vocab;

fn bypass_generated_validation() -> NounInstance {
    NounInstance(NounInstanceKind::Mass(Noun::Word(Vocab::Card)))
}

fn main() {}

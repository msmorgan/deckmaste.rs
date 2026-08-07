use deckmaste_english::syntax::NominalPhrase;
use deckmaste_english::word::NounInstance;

fn bypass(mut nominal: NominalPhrase, head: NounInstance) {
    nominal.head = head;
}

fn main() {}

use deckmaste_english::syntax::NounPhrase;
use deckmaste_english::syntax::Quantity;

fn bypass(quantity: Quantity) -> NounPhrase {
    NounPhrase::Quantity(quantity)
}

fn main() {}

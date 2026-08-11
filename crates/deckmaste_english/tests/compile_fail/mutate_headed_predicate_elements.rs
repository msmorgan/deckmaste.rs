use deckmaste_english::syntax::HeadedPredicate;
use deckmaste_english::syntax::Intransitive;

fn bypass(predicate: &mut HeadedPredicate<Intransitive>) {
    predicate.elements.clear();
}

fn main() {}

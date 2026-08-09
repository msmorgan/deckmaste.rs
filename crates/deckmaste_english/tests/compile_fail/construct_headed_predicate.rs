use deckmaste_english::syntax::HeadedPredicate;
use deckmaste_english::syntax::Intransitive;
use deckmaste_english::syntax::PredicateElement;
use deckmaste_english::syntax::PredicateHead;

fn bypass(head: PredicateHead) -> HeadedPredicate<Intransitive> {
    HeadedPredicate {
        head,
        kind: Intransitive,
        elements: Vec::<PredicateElement>::new(),
    }
}

fn main() {}

use deckmaste_english::syntax::AttachedPredicate;
use deckmaste_english::syntax::AttachmentScope;
use deckmaste_english::syntax::Predicate;

fn bypass(scope: AttachmentScope<Predicate>) -> AttachedPredicate {
    AttachedPredicate { scope }
}

fn main() {}

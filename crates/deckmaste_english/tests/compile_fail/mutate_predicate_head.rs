use deckmaste_english::syntax::PredicateHead;
use deckmaste_english::word::VerbInstance;

fn bypass(head: &mut PredicateHead, verb: VerbInstance) {
    head.verb = verb;
}

fn main() {}

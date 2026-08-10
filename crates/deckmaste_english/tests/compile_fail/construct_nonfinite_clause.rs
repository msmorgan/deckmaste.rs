use deckmaste_english::syntax::GerundClause;
use deckmaste_english::syntax::InfinitiveClause;
use deckmaste_english::syntax::InfinitiveMarker;
use deckmaste_english::syntax::Predicate;

fn main() {
    let predicate = predicate();
    let _ = InfinitiveClause {
        negated: false,
        marker: InfinitiveMarker::To,
        predicate: Box::new(predicate.clone()),
    };
    let _ = GerundClause {
        predicate: Box::new(predicate),
        attachments: Vec::new(),
    };
}

fn predicate() -> Predicate {
    loop {}
}

use deckmaste_english::syntax::InfinitiveClause;
use deckmaste_english::syntax::InfinitiveMarker;
use deckmaste_english::syntax::Predicate;

fn bypass(predicate: Predicate) -> InfinitiveClause {
    InfinitiveClause {
        negated: false,
        marker: InfinitiveMarker::To,
        predicate: Box::new(predicate),
    }
}

fn main() {}

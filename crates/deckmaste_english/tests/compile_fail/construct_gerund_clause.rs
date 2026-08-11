use deckmaste_english::syntax::GerundClause;
use deckmaste_english::syntax::GerundClauseKind;
use deckmaste_english::syntax::Predicate;

fn bypass(predicate: Predicate) -> GerundClause {
    GerundClause(GerundClauseKind::Base {
        predicate: Box::new(predicate),
    })
}

fn main() {}

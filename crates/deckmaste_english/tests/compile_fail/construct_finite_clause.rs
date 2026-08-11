use deckmaste_english::syntax::FiniteClause;
use deckmaste_english::syntax::PredicateExpression;
use deckmaste_english::syntax::Subject;

fn bypass(subject: Option<Subject>, predicate: PredicateExpression) -> FiniteClause {
    FiniteClause { subject, predicate }
}

fn main() {}

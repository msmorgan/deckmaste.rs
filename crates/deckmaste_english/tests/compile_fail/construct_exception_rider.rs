use deckmaste_english::syntax::ExceptionRider;
use deckmaste_english::syntax::IndependentClause;

fn bypass(first: IndependentClause) -> ExceptionRider {
    ExceptionRider {
        first: Box::new(first),
        rest: Vec::new(),
    }
}

fn main() {}

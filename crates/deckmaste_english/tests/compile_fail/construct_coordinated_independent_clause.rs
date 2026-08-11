use deckmaste_english::syntax::ClauseCoordination;
use deckmaste_english::syntax::CoordinatedIndependentClause;
use deckmaste_english::syntax::IndependentClause;

fn bypass(
    first: Box<IndependentClause>,
    rest: Vec<ClauseCoordination>,
) -> CoordinatedIndependentClause {
    CoordinatedIndependentClause { first, rest }
}

fn main() {}

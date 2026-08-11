use deckmaste_english::features::Comma;
use deckmaste_english::features::Conjunction;
use deckmaste_english::syntax::ClauseCoordination;
use deckmaste_english::syntax::CoordinatedClauseMember;

fn bypass(
    conjunction: Option<Conjunction>,
    comma: Comma,
    member: CoordinatedClauseMember,
) -> ClauseCoordination {
    ClauseCoordination {
        conjunction,
        comma,
        member,
    }
}

fn main() {}

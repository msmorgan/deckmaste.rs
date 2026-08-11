use deckmaste_english::features::Conjunction;
use deckmaste_english::syntax::WithAttributeCoordination;
use deckmaste_english::syntax::WithAttributeMember;

fn bypass_checked_continuation(
    conjunction: Option<Conjunction>,
    member: WithAttributeMember,
) -> WithAttributeCoordination {
    WithAttributeCoordination {
        conjunction,
        member,
    }
}

fn main() {}

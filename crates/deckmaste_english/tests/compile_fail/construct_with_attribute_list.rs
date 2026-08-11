use deckmaste_english::syntax::WithAttributeCoordination;
use deckmaste_english::syntax::WithAttributeList;
use deckmaste_english::syntax::WithAttributeMember;

fn bypass_checked_list(
    first: WithAttributeMember,
    rest: Vec<WithAttributeCoordination>,
) -> WithAttributeList {
    WithAttributeList { first, rest }
}

fn main() {}

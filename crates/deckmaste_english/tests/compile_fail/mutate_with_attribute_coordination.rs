use deckmaste_english::syntax::WithAttributeCoordination;
use deckmaste_english::syntax::WithAttributeMember;

fn bypass_checked_continuation(
    continuation: &mut WithAttributeCoordination,
    member: WithAttributeMember,
) {
    continuation.member = member;
}

fn main() {}

use deckmaste_english::syntax::RestrictionMember;
use deckmaste_english::syntax::RestrictionRun;

fn bypass(first: RestrictionMember) -> RestrictionRun {
    RestrictionRun {
        first,
        rest: Vec::new(),
    }
}

fn main() {}

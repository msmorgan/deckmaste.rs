use deckmaste_english::syntax::ClauseAttachment;
use deckmaste_english::syntax::ComplexClause;
use deckmaste_english::syntax::ExceptionRider;
use deckmaste_english::syntax::IndependentClause;
use deckmaste_english::syntax::RestrictionMember;
use deckmaste_english::syntax::RestrictionRun;

fn main() {
    let _ = ComplexClause {
        matrix: Box::new(independent()),
        attachments: Vec::<ClauseAttachment>::new(),
    };
    let _ = ExceptionRider {
        first: Box::new(independent()),
        rest: Vec::new(),
    };
    let _ = RestrictionRun {
        first: restriction_member(),
        rest: Vec::new(),
    };
}

fn independent() -> IndependentClause {
    loop {}
}

fn restriction_member() -> RestrictionMember {
    loop {}
}

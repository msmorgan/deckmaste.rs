use deckmaste_english::syntax::AttachmentScope;
use deckmaste_english::syntax::ComplexClause;
use deckmaste_english::syntax::IndependentClause;

fn main() {
    let _ = ComplexClause { scope: edge() };
}

fn independent() -> IndependentClause {
    loop {}
}

fn edge() -> AttachmentScope<IndependentClause> {
    loop {}
}

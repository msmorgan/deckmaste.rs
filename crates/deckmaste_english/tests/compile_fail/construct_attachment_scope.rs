use deckmaste_english::syntax::AttachmentScope;
use deckmaste_english::syntax::ClauseAttachment;
use deckmaste_english::syntax::IndependentClause;

fn bypass(
    host: IndependentClause,
    attachment: ClauseAttachment,
) -> AttachmentScope<IndependentClause> {
    AttachmentScope {
        host: Box::new(host),
        attachment: Box::new(attachment),
    }
}

fn main() {}

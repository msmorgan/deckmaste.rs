use deckmaste_english::features::Comma;
use deckmaste_english::syntax::Attachment;
use deckmaste_english::syntax::AttachmentPosition;
use deckmaste_english::syntax::ClauseAttachment;
use deckmaste_english::syntax::ClauseAttachmentKind;

fn bypass(
    position: AttachmentPosition,
    comma: Comma,
    payload: ClauseAttachmentKind,
) -> ClauseAttachment {
    Attachment {
        position,
        comma,
        payload,
    }
}

fn main() {}

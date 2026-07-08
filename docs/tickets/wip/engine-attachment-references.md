---
needs: [engine-attach]
---
`Reference::AttachHostOf` / `AttachedTo` resolution ([CR#701.3]). A thin
follow-up over the attachment-relation storage that `engine-attach` builds:
once the relation exists, `AttachHostOf(r)` resolves to the permanent `r` is
attached to and `AttachedTo(r)` to what is attached to `r`. Seamed by
`engine-resolve-selections` (the `todo!` in `eval_reference`, resolve.rs).

---
needs: [engine-attach]
---
`Reference::AttachHostOf` / `AttachedTo` resolution ([CR#701.3]). A thin
follow-up over the attachment-relation storage that `engine-attach` builds:
once the relation exists, `AttachHostOf(r)` resolves to the permanent `r` is
attached to and `AttachedTo(r)` to what is attached to `r`. Seamed by
`engine-resolve-selections` (the `todo!` in `eval_reference`, resolve.rs).

## Done (satisfied by trunk — no new code)
Superseded before it was claimed: the sweep implementing
[Invalid authoring fizzles](../../decisions/invalid-authoring-fizzles.md)
replaced the `eval_reference` `todo!`/`expect` seams (including
`AttachHostOf`) with the null-guarded `unbound_ref(...)` fizzle, and a later
`Reference` cleanup deleted the `Reference::AttachedTo` variant, moving the
host→attachment direction onto `RelationPredicate::AttachedTo` / `Attachment`.
Current implemented state:
- `resolve.rs` — `Reference::AttachHostOf(inner)` reads `attached_to`,
  null-guarded, fizzling on an unattached object.
- `target.rs` — `RelationPredicate::AttachedTo` / `Attachment` implement the
  host↔attachment direction over the same `attached_to` store.
Covered by `eval_reference_attach_host_of` and
`relation_filters_match_attachment_and_host` (both passing). No `todo!` remains.

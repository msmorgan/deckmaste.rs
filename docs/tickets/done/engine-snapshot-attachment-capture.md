---
needs: []
---
**Engine: `LkiSnapshot` never captured `attached_to`, so attachment relations
can't be checked against a departed object.**

`filter_matches_snapshot` (`trigger.rs`) is the last-known-information matcher
for objects that have left ([CR#603.10] — some triggered abilities look back
at the game state before the event; [CR#704.8] — LKI is taken from before a
simultaneous state-based sweep). It panics on
`RelationPredicate::AttachedTo`/`Attachment` ([CR#701.3]) because the snapshot
record has no `attached_to` field, even though the **live** matcher has
supported those relations since `engine-attachment-references` landed.

This blocks any "when ~ leaves the battlefield, do X to what it was attached
to" trigger — an Aura's leaves-the-battlefield ability that needs its last
host.

**Why the owner was wrong.** Two done tickets each expected the other to do
it. `engine-filter-breadth` wrote that `attached` "waits on `engine-attach`
then wires on refresh"; the actual follow-up, `engine-attachment-references`,
scoped its own Done section to `Reference::AttachHostOf` and the live
`target.rs` matcher and never mentions the snapshot path. So the seam kept
naming a ticket that closed without covering it.

Fix: `LkiSnapshot` already captures the analogous `tapped` and counter fields
— add `attached_to` alongside them, populate it at capture time from the live
object, and have the `AttachedTo`/`Attachment` arm read it instead of
panicking.

Done when a departed Aura's leaves-the-battlefield trigger resolves against
its LKI-captured host, and an unattached departed object reads the relation as
false rather than panicking.

Effort: **S**.

## Done

Added `LkiSnapshot::attached_to` (`crates/deckmaste_engine/src/lki.rs`),
populated at capture from the live object alongside `tapped`/`counters`, and
replaced the `AttachedTo`/`Attachment` seam in `filter_matches_snapshot`
(`crates/deckmaste_engine/src/trigger.rs`) with arms that read it.

`AttachedTo` resolves the captured host against the live store. `Attachment`
is the inverse — it scans live objects whose `attached_to` still points at the
departed id, which [CR#704.8] licenses: LKI predates the simultaneous
state-based sweep that clears the relation. Unattached reads false.

Pinned by three `trigger.rs` tests: the captured host resolves, the inverse
relation sees a still-live attachment, and an unattached departed object reads
both relations false instead of panicking.

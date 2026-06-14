---
needs: [engine-attach]
---
Evaluate Named, Stat, Relation (controller/owner/opponent/attached),
StateFilter (Status, RelatedBy, Targets/TargetCount), and Ref(Reference)
filters in `target.rs` and `trigger.rs` snapshot matching. `HasCounter`
and `Designated` already read live.

The matchers unify on `target::matches_with(state, id, filter,
Option<watcher>)`; `attached` (AttachedTo/Attachment) waits on `engine-attach`
then wires on refresh; `RelatedBy` stays a seam (no CR#607 relation registry).
Design: docs/superpowers/specs/2026-06-13-engine-filter-breadth-design.md

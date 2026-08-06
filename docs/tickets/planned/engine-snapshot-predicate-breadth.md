---
needs: []
design: true
---
**Engine: scope the LKI-snapshot matcher's unevaluated `Predicate` leaves.**

`filter_matches_snapshot` (`trigger.rs`) is the last-known-information twin of
the live matcher `target::matches_with`. Its combinators are peeled by the
shared `walk_combinators` walker and `Where` is handled, but the remainder is
partial and the tail falls into a catch-all.

This is a **scoping ticket, not an implementation ticket.** The residual is
large and heterogeneous, and the point of the ticket is to split it correctly
rather than to build it. The enumeration below is the deliverable that makes
that split possible without re-deriving it.

The residual, enumerated against the live match arms:

- `Predicate::Kind(ObjectKind)` — no arm.
- `Predicate::PlayerStatCmp(PlayerAttr, Cmp, Count)` — no arm, and no
  player-attribute fields on the snapshot to read.
- `Predicate::FromSource(Arc<Predicate>)` — no arm; lifts a quality to the
  departed object's stack-ability source.
- `Predicate::Ref(r)` for every `Reference` except `This` — `You`, `It`,
  `That`, `EventObject`, `EventPatient`, `Target(n)`, `ControllerOf`,
  `OwnerOf`, `AttachHostOf` and the rest. The current arm matches only `This`.
- `CharacteristicPredicate::Subtype`, and `Has(KeywordRef)` ([CR#702]) — 2 of
  9 characteristic leaves; the other 7 already read `snapshot_face`, and these
  two look like the same one-line shape.
- `StatePredicate::Designated`, `Attacking`, `Blocking` ([CR#509.1a]),
  `Unblocked` ([CR#509.1h]), `WasPaidWith`, `WasCastWith` ([CR#118.9,702.34a])
  — 6 of 14 state leaves. The snapshot captures none of this data.
- `RelationPredicate::TeammateOf` ([CR#102.4]) — structurally a one-line
  sibling of the existing `OpponentOf`/`Controls` arm.

**Why it splits three ways.** These are not one feature:

1. **Trivial dispatch gaps** — `TeammateOf`. No data missing, just an unwired
   arm parallel to one that exists.
2. **Snapshot-capture gaps** — the six state leaves, blocked on the snapshot
   growing fields. That is `engine-lki-robustness` territory, which
   investigates a generic property map precisely so new captured facts don't
   need per-predicate struct surgery. `engine-snapshot-attachment-capture` is
   the same class, already split out.
3. **Open semantics** — `PlayerStatCmp`, `FromSource`, `Kind`, and the
   non-`This` references. It is not obvious what several of these should
   *mean* against a snapshot of a gone object, so each needs a decision, not
   wiring.

Done when each class is split into its own ticket or folded into
`engine-lki-robustness` with this list carried forward, and `Predicate::Ref`
has a ruling on which references are even legal in a trigger-condition
position versus which should be rejected at authoring time rather than
surfacing as a runtime seam.

Effort: **S**.

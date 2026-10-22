---
needs: []
---
**Engine: `LkiSnapshot` captures the Object's derived characteristics and
Controller at capture time, and every snapshot read uses them.** Promoted from
`maybe/` on 2026-10-21: the original investigation (below) is now a correctness
defect, because layers 2 and 4-7 exist and the snapshot still reads the printed
card.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #8
(and the snapshot halves of #2 and #16).

## What is wrong at head

- `crates/deckmaste_engine/src/lki.rs:16-19`, `LkiSnapshot` doc: "Characteristics
  (types/P-T/name) derive from `source` via the card definition — correct in
  Stage 3 (no continuous effects yet to have modified them)." Stale. The struct
  stores no characteristics; `capture` (`:48-60`) stores the base
  `o.controller`.
- `trigger.rs:1586-1589` `snapshot_has_type` and `:1591-1625` `snapshot_stat`
  read `snapshot_face` (the printed face): "The snapshot stores no derived P/T, so
  this is the printed face — an LKI-stat capture seam." An animated land that dies
  fails "whenever a creature dies"; a creature that lost a type still matches it.
- `resolve/count.rs:36-66` `last_known_stat`: printed P/T plus +1/+1 and -1/-1
  counters. Canon Fling ignores Giant Growth or an anthem on the sacrificed
  creature.
- The snapshot test `snapshot_stat_reads_printed_stats` (`trigger.rs:5491`) pins
  the printed read; it is rewritten to assert a derived read (a pumped creature's
  snapshot reports its pumped power).

[CR#603.10]: look-back triggers use "the appearance of objects immediately prior
to the event". [CR#608.2h]: an effect needing information from an object that
left its expected public zone "uses the object's last known information". Fling's
ruling (2024-04-12): "Use the sacrificed creature's power as it last existed on
the battlefield."

## Change

Capture, at `LkiSnapshot::capture`, the derived values from `state.layers()`:
card types, subtypes, supertypes, colors, name, mana value, power, toughness,
abilities (keywords included, for the [CR#702.15c,702.2e] damage reads in
`engine-damage-results-life-events`), Controller, and the current Card Face Side.
`snapshot_has_type`, `snapshot_stat`, `last_known_stat` and the other
`snapshot_face` readers read the stored values. Delete the stale doc paragraph.

Keep the original constraint: the snapshot is a value (no live object
references; the engine never retains dead objects). Whether the captured set is
an enumerated struct or a generic per-characteristic map is the implementer's
call; the original investigation preferred a map so new captured facts do not
need per-predicate struct surgery (`engine-snapshot-predicate-breadth` class 2,
`engine-snapshot-object-classification`, `engine-snapshot-combat-state` all add
captured facts and should land on the same shape).

Overlap: `engine-statof-reference-channel` covers routing `Count::StatOf` through
a live-or-LKI reference and already asks for "derived power/toughness rather than
silently falling back to printed values"; this ticket supplies the captured
values that channel reads.

## Tests

- A creature pumped by an anthem is sacrificed to Fling; Fling deals the pumped
  power.
- An animated land (layer-4 creature type) dies; "whenever a creature dies"
  triggers.
- A stolen creature (layer-2) dies; its snapshot's Controller is the thief.

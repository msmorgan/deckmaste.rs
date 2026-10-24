---
needs: []
---

# english-v4: re-scan engine hygiene

Implements group B of the post-S7 re-scan (Argon, 2026-10-24): HIGH-2, HIGH-4,
MEDIUM-2, simplification 5 and the §4.5 gate index. Each finding is an
external-audit hypothesis; verify it by witness before acting on it.

## Change

- **HIGH-2.** `Production::priority` counts a PP as a preposition extension
  only when the PP's frame could fill a PP slot under `slot_equations`, so
  *Players attack with their hands revealed.* reads
  (`play_with_absolute.txt:13`).
- **HIGH-4.** Prove `unwritten_period` (Summary and State) is never true over
  the full census, then delete it. Refuted by measurement: the gate fires,
  and without it six faces with a quoted keyword verbal cost (*have
  "Ward—Pay 2 life."*) also read *…life..* and fail round-trip. It stays,
  documented and indexed in §4.5.
- **MEDIUM-2.** Merge `Summary::fronted` and `Summary::preposed_predicate`
  into one fronting enum.
- **Simplification 5.** Move `Expandable` and `SetOff` out of `ClauseKind`
  into frame flags.
- **§4.5 index.** Add the unlisted S7 runtime gates to
  `docs/decisions/english-v4.md` §4.5 as plain index entries.

## Done when

- HIGH-4, MEDIUM-2 and simplification 5 leave the census byte-identical.
- HIGH-2 admits the `play_with_absolute.txt:13` witness; every other changed
  unit is reported. No face is lost.

Standard constraints apply.

1. **Census delta** (`qqvuslxr` → `zmvnkwpo`, full corpus): faces read 32,769 → 32,769; total Readings 344,348 → 344,439 (6 units more, none fewer: Protective Sphere, Cemetery Prowler, Cemetery Illuminator, Holistic Wisdom, Cryptic Gateway, Mana Maze, each gaining the Absolute Clause *with*-Adjunct of *share*). Lost faces: none.
2. **Judgment set** (`zmvnkwpo`): 3,215 enforced, 317 pending; all hold.

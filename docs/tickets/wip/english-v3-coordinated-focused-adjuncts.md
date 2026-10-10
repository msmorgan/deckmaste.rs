---
needs: []
---
# Coordinate focused Adjuncts: Activate only as a sorcery and only once each turn.

## Why

A single focused Adjunct reads (*Activate only as a sorcery.*, *Activate only
once each turn.*); two of them joined by *and* or *but* do not. Named as a
residue in `english-v3-systemic-residuals` ("Once-frequency residue"). Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.json`): hidden
under `at:turn .` (about 25 of 43), `at:sorcery .` (14 of 16), `at:step .`,
`at:graveyard .`, `at:once .`. `cargo xtask english-v3 grep '\b(and|but) only\b'`
finds **82** faces, **57** unread; 57 failing lines match, **54** of them the
only failing line of their face (*and only* 45, *but only* 9). The 25 covered
faces are all *only during … and only if …* (Defiant Stand, Hakim,
Loreweaver). Sole lines by coordinate pair: *if* + *as a sorcery* 10, *during*
+ *once* 7, *if* + *once* 6, *as* + *once* 6, *as* + *if* 6, *once* + *if* 4,
others 3. So one order of one pair reads and the rest do not.

## Goal

A coordination of focused Adjuncts, of like or unlike category, reads as one
Adjunct of the clause: *only* PP + *and only* AdvP / PP / *if*-PP, and *but
only* PP after a complete clause, with byte-exact realisation.

## Analysis

CGEL Ch. 15 §3.2, p. 1326: "a difference of category is generally tolerated
where there is likeness of function". That backs coordinating a PP (*only as
a sorcery*) with an AdvP (*only once each turn*) as Adjuncts. Ch. 6 §7.3.1,
p. 587, [41]–[42] (cited by `FocusedPhrase`) backs each focused coordinate.
Treating *but only …* after a clause without its own *only* as the same
coordination is a project ruling.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`): schema
`FocusedPhrase` with instances for `NounPhrase`, `PurposeInfinitival`,
`PrepositionPhrase` and `AdverbPhrase`, each exporting `FocusedUse = Yes`;
the coordination schemas already carry `FocusedUse` through `AdverbPhraseSeries`
and the PP series, and "Activate during your turn and during your upkeep."
reads 1. What is missing is a coordination whose coordinates are focused and
may differ in category.

Orchestrator rulings 2026-10-10:

- CF1. First identify the route that reads the 25 covered *only during … and
  only if …* faces, and record its tree. If it is a coordination of focused
  Adjuncts, generalise it (order, AdvP coordinate, *as*-PP coordinate); no new
  Construction. If it is some other analysis (the *if* PP attached elsewhere,
  or *and only if* as a clause opener), STOP with both trees before adding a
  coordination beside it: which one those faces keep is a ruling.
- CF1a. The licence reads `FocusedUse` on each coordinate (both `Yes`) and the
  coordinates' Adjunct function licences. No test on *only*, *once* or
  *sorcery*.
- CF2. One analysis per string. *Activate only as a sorcery and only once each
  turn.* is one clause with a coordinated Adjunct. It must not also read as a
  clause coordination with an elided second *activate*, nor with the second
  *only* focusing into the first coordinate.
- CF3. *X but only Y* (*Any player may activate this ability but only as a
  sorcery.*, 9 sole faces): the first coordinate is the clause without a
  focused Adjunct. In scope if the same coordination reads it with *but* and a
  single focused coordinate attached to the clause; if it needs a distinct
  construction, list it with the count and leave it.
- CF4. Three coordinates with commas (*only during the declare blockers step,
  only if …, and only once each turn*, Grizzled Wolverine) follow from the
  series schema or stay out; do not add a construction for one face.
- CF5. Unfocused first coordinate with *and* ("Activate only as a sorcery and
  once each turn." 0) is unattested: no work, and no check to block it.
- CF6. Newly covered faces are gains. Reading-count increases are possible
  only on the 25 covered `(and|but) only` faces, as a second analysis of the
  same string; under CF1 that is a STOP, not a result to report after the
  fact. Any other increase is a defect.

## Witnesses

Each is the last sentence of the face's only failing line at `ymqwzour`.
Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`): all 0. Re-probe on the claim parent first. Not probed (the binary stopped
loading plugin data when trunk moved): the covered order "Activate only during
your upkeep and only if Hakim isn't enchanted." (census: its face is covered)
and the reversed orders; probe them before designing.

- Thunderhead Gunner: "Activate only as a sorcery and only once each turn."
- Instill Energy: "Activate only during your turn and only once each turn."
- Temple of Cyclical Time: "Activate only if it has no time counters on it and
  only as a sorcery."
- Security Detail: "Activate only if you control no creatures and only once
  each turn."
- Scandalmonger: "Any player may activate this ability but only as a sorcery."

Also 0 today: "Only your opponents may activate this ability and only as a
sorcery." (Detention Vortex); "Any player may activate this ability but only
during their draw step." Controls that must not change: "Activate only as a
sorcery." 1; "Activate only once each turn." 1; "Activate only if you control
no creatures." 1; "Activate during your turn and during your upkeep." 1; "Any
player may activate this ability." 1; "Only your opponents may activate this
ability." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bonly\b` case-insensitively (the surface the change can touch;
1,473 faces) plus 300 random covered controls (`cargo xtask english-v3
diff --manifest … --random 300 --seed 20261010`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against the
claim parent. Zero lost; CF6 for increases. Full `cargo test -p
deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests -- -D
warnings`, once, at the end: a rebuild after a grammar edit takes 2–3 minutes
and the six test binaries about 75 seconds, so batch edits. Tests assert the
role (each focused phrase a coordinate, the coordination an Adjunct of the
one clause, the CF2 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *Activate only during your turn, before attackers are declared.*
  (`english-v3-supplementary-temporal-adjunct`).
- *only any time they could cast a sorcery* (Teferi, 2 faces): temporal NP
  focus.
- *only if* + clause content that fails for its own reason (*seven or more
  cards in your graveyard* reads; *you've cast three or more instant and/or
  sorcery spells* on Urabrask may not): report, do not chase.

## Landing record

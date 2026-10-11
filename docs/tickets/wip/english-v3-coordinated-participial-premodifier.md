---
needs: []
---
# Coordinate an Adjective with a participle before a noun: create a tapped and attacking token that's a copy of it.

## Why

*tapped and attacking* reads as a Predicate ("This token enters tapped and
attacking." 1; "Create a token that's tapped and attacking." 1) and each word
reads alone as a premodifier ("Create a tapped 1/1 red Devil creature token."
1; "Create an attacking 1/1 red Devil creature token." 1). The coordination
does not read before a noun. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`): key `at:and attacking` 13
sole-blocker faces. `cargo xtask english-v3 grep '(a|an|two|three|X) tapped
and attacking'` finds **17** faces, all unread; 17 failing lines match,
**15** of them the only failing line of their face. *tapped and attacking*
overall is on 115 faces, 80 covered (the predicative use).

## Goal

A syndetic coordination of a state Adjective and a gerund-participle reads as
one attributive Modifier of a Nominal, before the rest of the premodifiers
(*a tapped and attacking 1/1 red Devil creature token*).

## Analysis

CGEL Ch. 15 §3.2(d), p. 1328, "Modifier in NP structure": "Mixed
coordinations in pre-head (attributive) modifier position, however, are not
common"; they "tend to involve adjectives and nominals belonging within a
single semantic set", [17iib] "*the state and federal laws*". That backs
unlike-category attributive coordination as possible and restricted. Admitting
Adjective + participle for the combat-state set is a project ruling.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`PremodifierCoordination` (category and feature), policies
`NounPremodifierConcord` and `AdjectiveStructureMerge` (`export
PremodifierCoordination = Yes`, `AdjectiveOrigin =
coordinated_adjective_origin(…)`), series `AdjectivePhraseSeries` and
`VerbalPremodifierSeries`. Like-category pairs read: "Destroy target
attacking or blocking creature." 1; "Tap target white or blue creature." 1.
The mixed pair fails in every host: "Destroy target tapped or blocking
creature." 0 (Tetsuo Umezawa, attested); "Tapped and attacking creatures get
+1/+1." 0.

Orchestrator rulings 2026-10-10:

- CP1. Refine the premodifier coordination so a coordinate from
  `AdjectivePhraseSeries` and one from `VerbalPremodifierSeries` may combine;
  no new Construction beside it. `coordinated_adjective_origin` (or its
  verbal counterpart) gets the mixed row.
- CP2. The licence reads the coordinates' declared premodifier-use features.
  No test on *tapped*, *attacking* or *token*. Do not add a "combat state"
  feature whose only job is to block other mixed pairs.
- CP3. One analysis per string. *a tapped and attacking token*: one
  coordinated Modifier. It must not also read as NP coordination (*a tapped*
  + *attacking token*), and *tapped* must not double as participle and
  Adjective inside the coordination: whichever category the bare premodifier
  *tapped* has today is the one the coordinate has.
- CP4. *or* (*target tapped or blocking creature*, *target attacking,
  blocking, or tapped creature*: Tetsuo Umezawa, Sonar Strike) is the same
  coordination and in scope; report the faces.
- CP5. Newly covered faces are gains. Reading-count increases are possible on
  covered faces where an Adjective and a participle premodifier are joined
  before a noun and currently read some other way; report each with its
  shape. The 80 covered predicative *tapped and attacking* faces must not
  change.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Furnace-Blessed Conqueror: "Whenever this creature attacks, create a tapped
  and attacking token that's a copy of it." (0; *a tapped token that's a copy
  of it* 3)
- Pugnacious Pugilist: "Whenever this creature attacks, create a tapped and
  attacking 1/1 red Devil creature token with "When this token dies, it deals
  1 damage to any target."" (0)
- Krothuss, Lord of the Deep: "Create a tapped and attacking token that's a
  copy of another target attacking creature." (0)
- Tetsuo Umezawa: "{U}{B}{B}{R}, {T}: Destroy target tapped or blocking
  creature." (sentence 0)

Controls that must not change: "This token enters tapped and attacking." 1;
"It's tapped and attacking." 1; "Create a 1/1 red Devil creature token that's
tapped and attacking." 3; "Create a tapped 1/1 red Devil creature token." 1;
"Destroy target attacking or blocking creature." 1; "Tap target white or blue
creature." 1; "Tap all attacking and blocking creatures." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\b(tapped|untapped|attacking|blocking|blocked|enchanted|equipped) (and|or) [a-z]+` or
`[a-z]+ (and|or) (tapped|untapped|attacking|blocking|blocked)\b` (coordinations with a state word)
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
CP5 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (the coordination as a single attributive Modifier of the Nominal,
each coordinate's category, the CP3 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *create a token that's a copy of … and that's tapped and attacking*
  (Flamerush Rider, Gyrus): coordinated relatives.
- *a copy of Shredder tapped and attacking that player* (postpositive with an
  Object).
- *put … onto the battlefield tapped and attacking* on faces with another
  blocker (16 touched faces).

## Landing record

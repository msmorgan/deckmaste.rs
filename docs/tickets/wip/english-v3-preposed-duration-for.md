---
needs: []
---
# Read a preposed duration *for as long as* Adjunct: For as long as that card remains exiled, its owner may play it.

## Why

Duration *for as long as* + clause reads at the end of a clause ("Its owner
may play it for as long as it remains exiled." 2) but not preposed with a
comma. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`): hidden under `at:it .` 6, `at:"
<end>` 3, `at:control .`, `at:cost .`, `at:block .`. `cargo xtask english-v3
grep 'For as long as'` (capitalised, so sentence-initial) finds **28** faces,
all unread; 28 failing lines match, **25** of them the only failing line of
their face. Lower-case *for as long as* is on 183 faces, 154 covered. Clause
shapes after the preposed form (failing-line faces): *that card remains
exiled* / *this creature remains tapped* 18, *… has a … counter on it* 6,
*you control …* 2, others 2.

## Goal

A *for* PP whose Complement is the equative duration phrase *as long as* +
clause reads as a preposed Adjunct, followed by a comma, of an imperative or
declarative clause, with the Readings the same PP has clause-finally.

## Analysis

CGEL Ch. 7 §5.1(d), p. 640 (cited by `AdverbComplementPreposition`) and
Ch. 13 §4.5(d), p. 1134 back the duration analysis already landed
(`english-v3-as-long-as`); this ticket relies on no further passage.
Preposing it is a project ruling. Glossary: **Duration Adverb**, **Scalar
Equality**, **Preposition Function Licence**.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`AdverbComplementPreposition` (`require head.AdverbComplementClass =
Duration; require complement.DurationUse = Yes`), with *for* declared
`AdverbComplementClass: Duration` and `PrepositionFunctionLicence:
…|Adjunct|…|PreposedAdjunct` in `lexicon/core.ron`. So the lexeme already
licenses the position (`licence_preposed_adjunct`); the PP built by this
construction does not reach it. Conditional *as long as* (`vocab:Preposition/
AsLongAs`, `Adjunct|PreposedAdjunct`) reads preposed: "As long as that card
remains exiled, you may play it." 1.

Orchestrator rulings 2026-10-10 (the 2026-10-06 ruling in
`english-v3-as-long-as` stands: scalar equality with *long* is admitted only
inside *for*'s duration Complement, and conditional *as long as* is a separate
Compound Preposition):

- PD1. Find which export of `AdverbComplementPreposition` keeps its PP out of
  the preposed-Adjunct position (candidates: it exports
  `ClauseComplementBoundary = None` although its Complement ends in a finite
  clause; `InterveningAdjunctLicence`; the comma rule) and correct that
  export. No new Construction.
- PD2. The licence is *for*'s declared `PreposedAdjunct` and the Complement's
  `DurationUse`. No test on *long* or *for*.
- PD3. One analysis per string. *For as long as X, Y* is the duration PP
  preposed. It must not also read as *for* + conditional *as long as*, nor as
  conditional *as long as* with a stray *for*; "As long as X, Y." keeps
  exactly its Readings and gains no duration twin.
- PD4. The preposed PP attaches to the clause, once: not also to each
  coordinate of *loses all abilities and can't attack or block* (Immovable
  Rod). Report coordination scope pairs if they arise; they are the existing
  preposed-Adjunct behaviour, not a new one.
- PD5. Newly covered faces are gains. Reading-count increases on the 154
  covered *for as long as* faces are not expected (all clause-final) and are
  reported with their shape; an increase on a covered *As long as …,* face is
  a defect under PD3.

## Witnesses

Each is the last sentence(s) of the face's only failing line at `ymqwzour`.
Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Prowl, Stoic Strategist: "For as long as that card remains exiled, its
  owner may play it." (0)
- Release to the Wind: "For as long as that card remains exiled, its owner
  may cast it without paying its mana cost." (0)
- Preacher: "For as long as this creature remains tapped, gain control of
  target creature." (0; the face adds *of an opponent's choice they control*)
- Minas Morgul, Dark Fortress: "For as long as that creature has a shadow
  counter on it, it's a Wraith in addition to its other types." (0)
- Flameshape: "For as long as they remain exiled, you may play them if you
  control a Wizard." (0)

Also 0 today: "For as long as you control this creature, it can't attack."
Controls that must not change: "Its owner may play it for as long as it
remains exiled." 2; "Gain control of target creature for as long as you
control this creature." 1; "As long as that card remains exiled, you may play
it." 1; "As long as that creature has a shadow counter on it, it's a Wraith
in addition to its other types." 3; "Until end of turn, you may play that
card." 1; "That card remains exiled." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `as long as` case-insensitively (duration and conditional: PD3
is about their boundary)
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
PD5 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (the *for* PP as preposed Adjunct of the main clause, *as long as* +
clause as its duration Complement, the PD3 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *it has "…"* after the comma where the quoted ability fails for its own
  reason (Mathas, Obsidian Fireheart, Makeshift Mannequin: `at:" <end>`).
- *It can't phase in for as long as you control this Saga.*
  (`english-v3-phase-in-out-verb`).
- Clause-final *for as long as* faces that are unread for another cause (29
  unread lower-case faces).

## Landing record

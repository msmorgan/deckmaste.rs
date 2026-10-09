---
needs: []
---
# Let a cardinal-measured *damage* take a Determiner and postmodifier: Prevent the next 3 damage that would be dealt to any target this turn.

## Why

*N damage* reads only as a closed NP. On census `qsupksky` (default `@` `mqrlzszk` is empty; 32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket "prevent the
next N damage" touches **118** unread faces and is the sole cause on **100**.
Counts are unread faces *touched* (at least one localised failing unit matches) / *sole* (every failing unit matches and no other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (`--readings 1`): "Prevent 3 damage." 1; "Prevent the 3 damage." 0;
"Prevent 3 damage that would be dealt to you this turn." 0; "Prevent the next
damage that would be dealt to any target this turn." 1; "Exile the next three
cards." 1.

## Goal

A cardinal-measured *damage* reads as a Nominal, so a Determiner (*the
next 3 damage*) and a relative postmodifier (*3 damage that would be dealt …*)
attach, with byte-exact realisation. *deals 3 damage* keeps its Readings.

## Analysis

Existing route: *3 damage* reads today only through the closed
`MeasuredNounPhrase` (`crates/deckmaste_english_v3/src/declarations.rs:3988`,
`[quantity: MeasurePhrase, head: lexical(Noun)]`, requires
`MeasurePosition = Before`), which takes no Determiner and no postmodifier.
Both `CardinalMeasuredNominal` (`declarations.rs:2548`, admits Determiners
and postmodifiers) and `CardinalMeasuredNounPhrase` (`declarations.rs:2576`)
require `CardinalMeasurementUse = Yes`, declared only on *mana*
(`crates/deckmaste_lexical_source/lexicon/core.ron:167`); *damage* has
`MeasurePosition: Before` only (`core.ron:168`).

Orchestrator ruling 2026-10-08 (R7a): declare `CardinalMeasurementUse: Yes`
on *damage* in the lexicon; no new Construction. *the next 3 damage that
would be dealt …* then reads through the route *3 mana* uses
(`CardinalMeasuredNominal` under a Determiner). No CGEL citation is added;
the existing analysis is reused unchanged.

Watch the bare string: with the feature, *deals 3 damage* becomes eligible
for `MeasuredNounPhrase`, `CardinalMeasuredNounPhrase` and
`CardinalMeasuredNominal` alike. First measure what *3 mana* yields today
(*Add 3 mana.*). If *3 damage* gains a second label for one constituency that
*3 mana* does not already have, that is a spurious duplicate (Method 8);
resolve it by restricting or retiring the redundant closed route, not by a
guard naming *damage*. A duplicate *3 mana* already carries is inherited:
record it, do not widen it.

## Witnesses

Each is the face's only failing unit at `qsupksky` (0 Readings), each
control (*the next damage …*) 1; (D) = `--category Document`:

- Hold at Bay: "Prevent the next 7 damage that would be dealt to any target
  this turn."
- Esper Battlemage (D): "{W}, {T}: Prevent the next 2 damage that would be
  dealt to you this turn."
- Anoint: "Prevent the next 3 damage that would be dealt to target creature
  this turn."
- Barbed Wire (D): "{2}: Prevent the next 1 damage that would be dealt by this
  artifact this turn."
- Kitsune Healer (D): "{T}: Prevent the next 1 damage that would be dealt to
  any target this turn." 

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose raw text contains case-insensitive " damage", plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, 12 workers and `--samples-per-face 0`; compare
   it with `cargo xtask english-v3 diff` against the claim parent's census of
   the same manifest. Never run `--all`. Tests: `cargo test -p` and
   `cargo clippy -p … --tests -- -D warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3`. No xtask tests, no
   Lean checks, no post-refresh re-census unless refresh touched grammar,
   lexicon or core. New tests go under tests/suite/ registered in
   tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Probe each witness before implementation and record the counts; write each
   witness as a test. Run the tests and clippy once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified
   in the landing record; a decrease inherited from trunk is attributed by
   change id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *3 damage* reading both as the closed NP and as the determinerless Nominal NP on the same string (a spurious duplicate), or *deals 3 damage* gaining Readings, is a defect; restrict or retire the closed route per Method 8. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule); check attestation with `cargo xtask english-v3 grep`.
   Data over code: a declaration an existing Construction already reads is
   preferred to a grammar change.
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests. Prefer refining an existing Construction: the count is above the
   ADR's 250 ceiling.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- Other measure nouns (*life*) unless attested on a face this ticket covers.
- *Prevent all but N damage*.

## Landing record

1. **Census delta**: bounded before → after (covered faces, Readings); lost
   faces classified by name; decreases, increases, digest-only changes.
2. **Witnesses**: each witness's Reading count before and after.
3. **Spot checks**: ten newly covered faces with their selected Reading.
4. **Deviations and STOPs**: constructions or tests added or removed beyond
   the ticket's letter, with justification; every STOP and its resolution;
   any glossary gap.
5. **Tests**: commands and outcomes; restored, re-spelled, ignored with
   blockers, added, removed.
6. **Timing**: one bounded census wall time with worker count.

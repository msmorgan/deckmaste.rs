---
needs: []
---
# Read a comparative property value: creatures with power less than this creature's power.

## Why

*with power less than X* never reads although *with power 2 or less* and
*its power is less than X* do. On census `qsupksky` (default `@` `mqrlzszk` is empty; 32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket "with <property>
less/greater than" touches **143** unread faces and is the sole cause on
**114**. Counts are unread faces *touched* (at least one localised failing unit matches) / *sole* (every failing unit matches and no other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (`--readings 1`): "Destroy target creature with power less than 3." 0,
"… with power 3 or less." 1; "Its power is less than 3." 1.

## Goal

After a property noun (*power*, *toughness*, *mana value*), a comparative
AdjP (*less than X*, *greater than X*, *less than or equal to X*) reads as
the property value, with byte-exact realisation.

## Analysis

CGEL, Ch. 6 §3.3(c), p. 552, [46ii] (*a leader younger than you*):
postpositive AdjPs follow an ordinary noun head; Ch. 6 §2.1, pp. 528–529,
for the postpositive function. That backs the function only.

Existing route: `MeasuredAttribute`
(`crates/deckmaste_english_v3/src/declarations.rs:4047`,
`[head: Nominal, quantity: PropertyValue]`, category `PropertyValue` at
`declarations.rs:341`) reads *power 2 or less*. The comparative AdjP reads
predicatively through `ComparativeAdjectivePhrase` (`declarations.rs:4141`)
with `NominalComparativeComplement` (`declarations.rs:4137`).

Orchestrator ruling 2026-10-08 (R5a): the comparative enters as a
`PropertyValue`, through the existing property-value route, not as a new
Construction category or a new NP postmodifier construction. `PropertyValue`
is fed today by the `ScalarMeasureValue` schema instance row
`(PropertyValue, ScalarValue, ScalarPropertyValueKind)`
(`declarations.rs:4015–4018`) and the `ScalarValue` constructions after it;
widen that route so `MeasuredAttribute` reads *power less than X* the way it
reads *power 2 or less*. *less than or equal to X* belongs to the same value
slot. *whose power is less than 3* (relative-clause gap) is out of scope.

Distinct from the planned
`english-v3-postpositive-comparative-determinatives` (determinative *more*
after a measured NP); no dependency.

## Witnesses

Each is the face's only failing unit at `qsupksky` (0 Readings), each
control 1; (D) = `--category Document`:

- Howlgeist: "Creatures with power less than this creature's power can't
  block it." (control "Creatures with power 2 or less can't block it.")
- Unliving Psychopath (D): "{B}, {T}: Destroy target creature with power less
  than this creature's power." (control "… with power 2 or less.")
- Ensnaring Bridge: "Creatures with power greater than the number of cards in
  your hand can't attack." (control "Creatures with power 2 or greater can't
  attack.")
- Immobilizer Eldrazi (D): "{2}{C}: Each creature with toughness greater than
  its power can't block this turn." (control "… with toughness 2 or greater
  …")
- Kiora, Sovereign of the Deep: "You may cast a spell with mana value less
  than X from among them without paying its mana cost." (control "… with mana
  value X or less …")

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose raw text contains case-insensitive "less than" or "greater than", plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, 12 workers and `--samples-per-face 0`; compare
   it with `cargo xtask english-v3 diff` against the claim parent's census of
   the same manifest. Never run `--all`. Tests: `cargo test -p` and
   `cargo clippy -p … --tests -- -D warnings` for every crate whose code or consumed data changed (`deckmaste_english_v3` always; `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed). No xtask tests, no
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
   Reading. The comparative read as a postmodifier of the whole NP (*creature with power*) rather than as the property value, or *than X* attached outside the AdjP, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *whose power is less than X* (relative-clause gap; its control also
  fails).
- Determinative *more* after a measured NP
  (`english-v3-postpositive-comparative-determinatives`).

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

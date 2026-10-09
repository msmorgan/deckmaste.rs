---
needs: []
---
# Read *at least* before a cardinal: you control at least two other Merfolk.

## Why

*at least N* never reads although *N or more* does. On census `qsupksky`
(32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of
2026-10-08), the recon bucket "at least" touches **102** unread faces and is
the sole cause on **78**. Counts are unread faces *touched* (at least one
localised failing unit matches) / *sole* (every failing unit matches and no
other recon STRONG bucket does); they are surface counts, not gain forecasts.
`cargo xtask english-v3 grep -i '\bat least\b'` finds the same 102 faces, all
unread at `qsupksky`; *at most* has 0 faces.

The planned `english-v3-postpositive-comparative-determinatives` has
witnesses of the shape *at least 10 life more than …* (Chalice of Life,
Righteous Valkyrie); this ticket unblocks their *at least* half. No `needs`
edge either way.

## Goal

*at least* reads as a lower-bound modifier of a cardinal wherever a cardinal
reads today: as a Determiner (*at least two other Merfolk*, *at least one
Angel card*) and inside a measure phrase (*at least 7 life*, *at least four
mana*), with byte-exact realisation.

## Analysis

CGEL Ch. 5 §11(e), p. 432, [7]: the PPs *at least* / *at most* mark lower and
upper bounds inside the DP and "occur mainly with numerals". That backs the
PP-modifier analysis only.

Existing route: `PrepositionDeterminer`
(`crates/deckmaste_english_v3/src/declarations.rs:4114`,
`[value: QuantitativePrepositionPhrase]`) already reads *up to two* through
the `QuantitativePrepositionPhrase` schema and its instance rows
(`declarations.rs:4053`, `:4070`; *up* declares
`QuantitativeComplement: CardinalPrepositionPhrase`, *to* declares
`QuantitativeComplement: Cardinal`, `lexicon/core.ron`). `CardinalDeterminer`
(`declarations.rs:4107`) and `DegreeDeterminativePhrase` (`:4135`, a
Determinative modifying a Determinative) are the neighbouring routes.
*least* is `vocab:Adjective/Least` (`lexicon/core.ron:1037`, no features);
*at* is `vocab:Preposition/At` (`core.ron:209`).

Orchestrator rulings 2026-10-09:

- R9a. *at least* is a PP Modifier of the cardinal (CGEL p. 432), not a fixed
  multiword Determinative lexeme and not a new Determinative. *least* keeps
  its Adjective lexeme; whatever licenses it as the Complement of *at* is a
  declared feature, never a word test.
- R9b. Extend the existing `QuantitativePrepositionPhrase` /
  `PrepositionDeterminer` family (a new instance row or a widened
  Complement) before adding a Construction. The bounded cardinal must stay
  one constituent so that measure phrases (*at least 7 life*) and Determiners
  share the route; if the measure position needs a second consumer, that is
  a refinement of the existing Cardinal consumer, not a parallel route.
- R9c. Pruning rule: *at most* is unattested (0 faces); do not license it.
- R9d. *at least N* must not also read as *at* + NP (*least two other
  Merfolk*) or as a clause-level *at* Adjunct; one analysis per string.
- R9e. Number follows the cardinal, as for *up to one* / *up to two*.

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0, control 1.

- Sokka, Lateral Strategist: "Whenever Sokka and at least one other creature
  attack, draw a card." (control "… one or more other creatures …")
- Svyelun of Sea and Sky: "Svyelun has indestructible as long as you control
  at least two other Merfolk." (control "… two or more other Merfolk.")
- Renegade Reaper: "If at least one Angel card is milled this way, you gain 4
  life." (control "If one or more Angel cards are milled this way, …")
- Synchronized Eviction: "This spell costs {2} less to cast if you control at
  least two creatures that share a creature type." (control "… two or more
  creatures …")
- Glimmer Lens: "Whenever equipped creature and at least one other creature
  attack, draw a card." (control "… one or more other creatures …")

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose analysed text matches case-insensitive `\bat least\b`, plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261009`),
   with the identity manifest, `--workers 6` and `--samples-per-face 0`, run
   under `/usr/bin/time -v`; compare it with `cargo xtask english-v3 diff`
   against the claim parent's census of the same manifest. Never run `--all`.
   Tests: `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
   every crate whose code or consumed data changed (`deckmaste_english_v3`
   always; `deckmaste_lexical_source` if `core.ron`, `verbs.ron` or
   `vocabulary.rs` changed). No xtask tests, no Lean checks, no post-refresh
   re-census unless refresh touched grammar, lexicon or core. New tests go
   under `tests/suite/` registered in `tests/main.rs`. Evidence lives under
   the workspace's ignored `target/english-v3/`, never `/tmp`.
2. Probe first: probe each witness and control before implementation and
   record the counts; write each witness as a test. Run the tests and clippy
   once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified
   in the landing record; a decrease inherited from trunk is attributed by
   change id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *at least* read as *at* + an NP headed by *least*, as a clause-level Adjunct, or detached from its cardinal is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit lexicon declarations in `crates/deckmaste_lexical_source/lexicon/`
   and `grammar:` frame fields in `plugins_v2` keyword actions; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule); check attestation with `cargo xtask english-v3 grep`.
   Data over code: a declaration an existing Construction already reads is
   preferred to a grammar change.
8. Refine an existing Construction before adding one: the Construction count
   is above the ADR's 250 ceiling. A new frame or Construction must not overlap
   an existing one on the same string (two labels for one constituency is a
   spurious duplicate, not an ambiguity); when it supersedes one, retire the
   old route and re-spell its tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- *at least that many*, *at least as many*, *at least one of* (1 face each).
- *at least two more* / *at least 10 life more than …*: the comparative half
  belongs to `english-v3-postpositive-comparative-determinatives`.
- *at most* (unattested).

## Landing record

Six items, each figure stamped with the change id it was measured on (ruling
2026-10-08):

1. **Census delta**: covered faces and total Readings before/after on the
   bounded manifest; lost faces by name with classification, or "none".
2. **Witnesses**: each witness above with its Reading count after.
3. **Spot checks**: ten newly covered faces by name with the selected analysis.
4. **Deviations and STOPs**: constructions, frames or tests added or deleted
   beyond the ticket's letter, with justification; every STOP and its
   resolution; any glossary gap in `docs/contexts/oracle-english/CONTEXT.md`.
5. **Tests**: the `cargo test -p` / clippy commands and outcomes; counts
   restored / re-spelled / ignored-with-blocker / added / removed.
6. **Timing**: wall time of the bounded census as one figure with worker count
   (6), plus peak RSS from `/usr/bin/time -v`.

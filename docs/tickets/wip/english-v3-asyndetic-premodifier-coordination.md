---
needs: []
---
# Read asyndetic premodifier coordination: Destroy target nonartifact, nonblack creature.

## Why

*nonartifact and nonblack creature* reads; *nonartifact, nonblack creature*
does not. On census `qsupksky` (32,828 supported faces, 25,474 covered, 7,354
unread; wave-5 recon of 2026-10-08), the recon bucket "comma-stacked negative
premodifiers" touches **83** unread faces and is the sole cause on **64**
(touched / sole as in the wave-5 recon; surface counts, not gain forecasts).
`cargo xtask english-v3 grep -i '\bnon-?[A-Za-z]+, non'` finds 83 faces
(*noncreature, nonland*; *nonartifact, nonblack*; *non-Vampire, non-Werewolf,
non-Zombie*; *nonland, nontoken*).

Probes (`--readings 1`): "Destroy target nonartifact, nonblack creature." 0;
"… nonartifact and nonblack creature." 1; "… nonartifact nonblack
creature." 1; "Destroy target red, green creature." 0.

## Goal

Two or more premodifiers joined by commas alone, before one head, read as an
asyndetic coordination of premodifiers with the same members and structure
as the syndetic *X and Y* reading.

## Analysis

CGEL Ch. 15 §1, p. 1276, [4]: coordination may be syndetic (*all his
colleagues and all his students*) or asyndetic (*all his colleagues, all his
students*); p. 1277 notes that asyndetic coordination is not always clearly
distinguishable from non-coordinative juxtaposition. That backs the
asyndetic analysis only.

Existing route: the `Coordination` schema
(`crates/deckmaste_english_v3/src/declarations.rs:1498`, separator by the
declared `CommaCoordination` feature, `:1495`) and its instance list
(`:2581`), whose rows include `NounPremodifier` and `AdjectivePhrase`;
`SerialCoordination` (`:1491`, `:2461`) and `CoordinationSeriesEnd`
(`:1482`) for longer series. Consumers: `NounPremodifiedNominal` (`:2618`)
and `PremodifiedNominal` (`:2632`). *nonartifact* lexes as the negative type
word (`lexeme:type/artifact/non`), *nonblack* as the negative colour word
(`vocab:ColorWord/Black/non`), so the conjuncts may be unlike categories,
exactly as in the syndetic reading that already reads.

Orchestrator rulings 2026-10-09:

- R15a. Extend the existing coordination schemas with a coordinator-less
  (asyndetic) form; do not add a stacked-premodifier-with-comma Construction.
  The asyndetic Reading has the same members and grouping as the syndetic
  one minus the coordinator.
- R15b. Pruning rule: open the asyndetic form only for the instance rows
  whose asyndetic use is attested in the supported corpus (measure with
  `cargo xtask english-v3 grep`; premodifier rows). Asyndetic NP, clause,
  predicate or PP coordination stays closed unless attested *and* measured
  at zero loss; *red, green creature* is unattested and must stay 0.
- R15c. One analysis per string: *nonartifact, nonblack creature* must not
  read both as an asyndetic coordination and as two stacked premodifiers
  separated by a comma (two labels for one constituency).
- R15d. Comma lists that end in a coordinator (*white, blue, or black*)
  keep their existing `CoordinationSeriesEnd` Reading count.

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0, control (the
first premodifier and its comma removed) 1.

- Terror: "Destroy target nonartifact, nonblack creature."
- Duress: "You choose a noncreature, nonland card from it."
- Druidic Satchel: "If it's a noncreature, nonland card, you gain 2 life."
- Victim of Night: "Destroy target non-Vampire, non-Werewolf, non-Zombie
  creature." (control "Destroy target non-Vampire creature.")
- Filter Out: "Return all noncreature, nonland permanents to their owners'
  hands."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose analysed text matches case-insensitive `\b[a-z-]+, [a-z-]+ (creature|permanent|card|land|spell|artifact|enchantment|planeswalker|token)s?\b|\bnon-?[a-z]+, ` (2,275 faces on 2026-10-09; it includes comma-separated words before a head noun, so overgeneration shows), plus 300 random covered controls
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
   Reading. A comma-separated premodifier pair read as stacked (non-coordinate) premodifiers, as NP or clause coordination, or with a grouping the syndetic *and* reading lacks, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- Asyndetic coordination outside premodifiers (unattested or unmeasured).
- Faces whose *noncreature, nonland* NP has another blocker (e.g. Blot Out
  the Sky: its control "If X is 6 or more, destroy all nonland permanents."
  also reads 0).

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

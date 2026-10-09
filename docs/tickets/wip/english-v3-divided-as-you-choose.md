---
needs: []
---
# Read *divided as you choose among*: deals 2 damage divided as you choose among one or two targets.

## Why

*divide* is not a word. On census `qsupksky` (32,828 supported faces, 25,474
covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket
"divided as you choose" touches **81** unread faces and is the sole cause on
**66** (touched / sole as in the wave-5 recon; surface counts, not gain
forecasts). Attestation (`cargo xtask english-v3 grep -i`): *divided* 81
faces, *divided as you choose* 78; *divide* 2 (Defensive Formation's finite
"You can divide that creature's combat damage as you choose among …"; Squall,
SeeD Mercenary's ability word *Rough Divide*). Shapes: *N damage divided as
you choose among any number of targets / one or two targets / one, two, or
three targets / up to N targets / those permanents / them*, *its power
divided as its controller chooses among …*, and a supplementive *…, divided
as you choose.* (6 faces).

Probes (`--readings 1`): every witness reports the vocabulary gap *divided*.
Diagnostics with the declared verb *distribute*: "Distribute two counters
among one or two target creatures." 1; "Distribute two counters as you
choose among one or two target creatures." 0; "Forked Bolt deals 2 damage
distributed among one or two targets." 0; "Two counters are distributed
among one or two target creatures." 0. So three pieces are missing: the verb,
its past-participial use with the *among* frame, and the *as you choose*
Adjunct before a frame Complement.

## Goal

*divide* is a verb with *distribute*'s attested *among* frame; *N damage
divided as you choose among X* reads as a Nominal with a past-participial
postmodifier whose internal Adjunct is *as you choose* and whose *among* PP
is the verb's frame Complement.

## Analysis

CGEL Ch. 5 §14.2(e), p. 446, [15iv] (*some letters written by her
grandmother*): past-participial clauses as NP-internal Modifiers. Ch. 13
§5.5, p. 1149, [34] ("Manner"): an *as* phrase whose Complement is a
comparative clause functions as manner element in its clause (*The louvres
are constructed [as — shown in the diagram]*). Those back the postmodifier
and the manner *as* phrase only.

Existing routes: `Distribute` (`crates/deckmaste_lexical_source/lexicon/verbs.ron:156–159`,
`[ObjectNounPhrase]` and `[ObjectNounPhrase, MarkedRole("Preposition",
"Among", "NounPhrase")]`, no `participle`); `Choose`
(`verbs.ron:22`, participle declared). `ParticipialPostmodifiedNominal`
(`crates/deckmaste_english_v3/src/declarations.rs:3681`, BarePassive past
participles; "Prevent all damage dealt to it this turn." reads 1).
*as* + finite clause is `ClauseComplementPreposition` (`declarations.rs:3308`,
`vocab:Preposition/As` declares `FiniteClauseComplement: Yes`,
`lexicon/core.ron:200`); the object-gap clause is `FiniteObjectGap`
(`declarations.rs:3920`; "Exile the card you choose." reads 1).

Orchestrator rulings 2026-10-09:

- R12a. Declare the verb *divide* (bare, third person, preterite and
  participle *divided*) with the attested *among* frame, copied from
  Distribute's; no new Role. Its plain transitive frame is declared only if
  attested (pruning rule; Defensive Formation is the one finite use).
- R12b. *as you choose* / *as its controller chooses* is an *as* phrase whose
  Complement is the existing gapped clause (`FiniteObjectGap`, *choose*
  taking its Object gap), attached inside the participial clause, not to
  *deals*. No intransitive *choose* frame, and no frame `Literal("as")`.
- R12c. If no existing route admits an Adjunct between a verb and its frame
  Complement, STOP with the measured options before adding one; do not solve
  it with a frame that names *as*.
- R12d. Why *distributed among …* fails as a participial postmodifier while
  *dealt to it* reads must be diagnosed first; if the cause is Distribute's
  missing `participle`, declaring *divided* correctly is the fix for this
  ticket, and Distribute's own participle is declared only if attested.
- R12e. Gaining *divided* must not give *damage* a second Reading for the
  same constituency (participle attached both to *damage* and to the clause).

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0 (gap *divided*),
control 1. (D) = `--category Document`.

- Forked Bolt: "Forked Bolt deals 2 damage divided as you choose among one or
  two targets." (control "Forked Bolt deals 2 damage to any target.")
- Magma Opus: "Magma Opus deals 4 damage divided as you choose among any
  number of targets." (control "… 4 damage to any target.")
- Samut, the Tested (D): "−2: Samut deals 2 damage divided as you choose
  among one or two targets." (control "−2: Samut deals 2 damage to any
  target.")
- Death to Our Enemies: "When you do, it deals 7 damage divided as you choose
  among one or two targets." (control "… 7 damage to any target.")
- Boulderfall: "Boulderfall deals 5 damage divided as you choose among any
  number of targets." (control "… 5 damage to any target.")

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose analysed text matches case-insensitive `\bdivid(e|es|ed)\b` or `\bas \S+( \S+)? chooses?\b`, plus 300 random covered controls
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
   Reading. *as you choose* attached to *deals* rather than inside the participial clause, or the *among* PP read as anything but the frame Complement of *divided*, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *divided evenly, rounded down, among …* (2 faces).
- The supplementive *…, divided as you choose.* after the target NP (6
  faces) unless it falls out with no extra Construction.
- Engine semantics of division (`engine-divided-distribution-as-you-choose`).

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

---
needs: []
---
# Read the existential clause: there are seven or more cards in your graveyard

## Why

Dummy *there* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *there is/are* touches **438** unread
faces and is the sole cause on **276** (recon bucket "there is/are"; it was 22
sole on the wave-2 recon, before the comparative-quantity, *as long as* and
cost buckets it co-occurred with were discharged). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "There is a creature." 0; "There are two
creatures." 0; "There are no creatures on the battlefield." 0; "This creature
can't attack unless there is a Mountain on the battlefield." 0, while "… unless
you control a Mountain." 1; "Enchanted creature gets +3/+0 as long as there are
seven or more cards in your graveyard." 0, while "… as long as you have seven
or more cards in your graveyard." 1.

This ticket takes over from `english-v3-systemic-residuals` the existential
*there* host obligation recorded for Shadowborn Demon ("No implementation
ticket owns existential *there*").

## Goal

A finite clause with dummy *there* as Subject, *be* as verb and a displaced
Subject NP reads, with an optional locative PP after the displaced Subject
(*on the battlefield*, *in your graveyard*, *on this land*). It reads in every
host where an ordinary finite Clause already reads: main clause, *if*/*when*
conditional and temporal clauses, *unless*, *as long as*. Finite *be* agrees
in number with the displaced Subject (*there is a Desert card*, *there are
four or more card types*); the mismatched forms do not read.

## Analysis

Dummy *there* is a pronoun filling the subject position while the element
that would be subject in the basic version is displaced to post-verbal
position, as a **displaced subject** that is an internal complement of the
verb (CGEL, Ch. 16, §6.1, p. 1391, [3]). Existential clauses are defined
syntactically as the combination of *there* and *be* (Ch. 16, §6.2, p. 1392);
presentationals with other verbs (*There remain many problems*, p. 1390) are
not attested in these witnesses and are not added (pruning rule). The
agreement claim in the Goal is the ticket's requirement; consult Ch. 16 §6
for its CGEL basis before citing it.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Sandstone Needle: "If there are no depletion counters on this land, sacrifice
  it." (control "If this land has no depletion counters on it, sacrifice it."
  1)
- Glacial Crasher: "This creature can't attack unless there is a Mountain on the
  battlefield."
- Kamahl's Desire: "Threshold — Enchanted creature gets +3/+0 as long as there
  are seven or more cards in your graveyard."
- Case of the Shifting Visage: "To solve — There are fifteen or more cards in
  your graveyard."
- Drop of Honey: "When there are no creatures on the battlefield, sacrifice this
  enchantment."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p deckmaste_english_v3` and clippy on it; plus
   `-p deckmaste_lexical_source` only if core.ron/verbs.ron changed. No xtask
   tests, no Lean checks, no post-refresh re-census unless refresh touched
   grammar or lexicon. New tests go under tests/suite/ registered in
   tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *there* read as a locative Adjunct or as Subject of an
   ordinary predicate, the displaced Subject read as an Object, or a
   singular/plural mismatch that reads, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.
11. Timings as integer ns and ns/B, with host load and worker count.

## Out of scope

- Presentational *there* with verbs other than *be*, and *there* raised under a
  catenative (*there seems to be*): unattested here.
- Relativised displaced Subjects (*everything there is*) and existential
  interrogatives.
- Other blockers on *card types among …* hosts (Delirium faces): measure them,
  do not build them here.

## Landing record

In addition to the standard record: the Subject/displaced-Subject structure
chosen and how agreement is enforced (declared features); Reading counts of
each witness and of the agreement-mismatch negatives; the residual item
discharged; before/after counts stamped with change ids; timings as integer ns
and ns/B with host load and worker count.

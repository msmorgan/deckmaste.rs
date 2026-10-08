---
needs: []
---
# Read *as though* with a content-clause Complement: cast spells as though they had flash

## Why

*as though* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *as though* touches **264** unread faces and
is the sole cause on **128** (recon bucket "as though"). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "You may cast spells." 1; "You may cast spells as
though they had flash." 0; "They had flash." 1; "It didn't have defender." 1;
"This creature can attack this turn." 1; "This creature can attack this turn
as though it didn't have defender." 0; "This creature can attack as if it
didn't have defender." 0.

## Goal

The compound preposition *as though* heads a PP whose Complement is a finite
content clause, read as a clause-final manner Adjunct, including under modal
and passive predicates (*can be cast as though it had flash*). The content
clause takes a preterite (*they had flash*, *it weren't blocked*) whatever the
matrix tense. *As if* is added only if it is attested in the supported
corpus (pruning rule).

## Analysis

*As if* and *as though* introduce phrases functioning, among others, as manner
adjunct (CGEL, Ch. 13, §5, p. 1151, [38i]). CGEL analyses *as* and *if*
(*though*) as merged into a single compound preposition taking a content clause
as complement (p. 1151). The construction allows irrealis *were* or a modal
preterite (p. 1152, [42]); the corpus *as though it weren't blocked* is that
irrealis *were*, so it needs a declared verb-form feature, not a word-named
guard.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Vedalken Orrery: "You may cast spells as though they had flash."
  (control "You may cast spells." 1)
- High Fae Trickster: "You may cast spells as though they had flash."
- Blur of Heroism: "You may cast Hero spells as though they had flash."
- Mirror Wall: "{W}: This creature can attack this turn as though it didn't
  have defender."
- Ruxa, Patient Professor: "For each creature you control with no abilities,
  you may have that creature assign its combat damage as though it weren't
  blocked."

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
   Reading. *as* read as a separate predicative preposition and *though*
   as a concessive, or the clause read as a relative, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *as though its power were 2 greater* (Cloudspire Captain): "Its power were 2
  greater." is 0 on its own; note it.
- *as if* with a *to*-infinitival (p. 1262, [22ii]); unattested.

## Landing record

In addition to the standard record: the compound-preposition analysis chosen,
the Adjunct attachment, the tense/irrealis handling (declared features);
whether *as if* is attested and added; Reading counts of each witness;
before/after counts stamped with change ids; timings as integer ns and ns/B
with host load and worker count.

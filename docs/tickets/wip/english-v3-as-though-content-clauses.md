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

## Out of scope

- *as though its power were 2 greater* (Cloudspire Captain): "Its power were 2
  greater." is 0 on its own; note it.
- *as if* with a *to*-infinitival (p. 1262, [22ii]); unattested.

## Landing record

1. **Census delta.** Claim-parent sources match `zpmurnqu`: **23,005 covered / 1,131,832 Readings**. Refreshed parent `pvmvwtuq`: **23,216 / 1,147,462**. Finished `qxqootwz`: **23,352 / 1,199,416** (**136 feature gains**). Lost faces and removed Readings: **none**; previously covered digests match both parents. A grammar/lexicon-changing refresh required the permitted final full re-census.

2. **Witnesses.** At `qxqootwz`: Vedalken Orrery **3**, High Fae Trickster **3**, Blur of Heroism **3**, Mirror Wall **8**, Ruxa, Patient Professor **160** full-face Readings. Ruxa's isolated witness at `tvknqzwq` has **20**. The compound head takes a finite content-clause Complement; the final manner Adjunct admits modal preterites and declared non-tensed irrealis forms. Unattested *as if* was omitted; Cloudspire Captain remains out of scope.

3. **Spot checks.** At `qxqootwz`: Vedalken Orrery, High Fae Trickster and Blur of Heroism — compound PP with *they had flash*; Mirror Wall and Bristlepack Sentry — *it didn't have defender*; Quicken — *it had flash* under a modal passive; Ruxa, Indomitable Might and Deathcoil Wurm — irrealis *it weren't blocked*; Mezzio Mugger — irrealis *it were mana of any color*. Each selected Reading has a finite content-clause Complement and final manner Adjunct. Four further gains need trunk's nominal comparison as well: Felothar, Assault Formation, High Alert and Arcades.

4. **Deviations and STOPs.** At `qxqootwz`, added **2 constructions**, **2 categories**, **2 existing-schema instances**, a typed irrealis form and **10 tests**, including independent construction, admission/roundtrip/traversal, licence removal, negatives and Quicken. No frames added. Retired standalone *though* and the compound licence on *as*. Fixed the existing existential-concord regression through a general tensed-form requirement; adapted PP exports after refresh. Defined glossary gaps: Content Clause, Irrealis, Irrealis Clause and Irrealis Clause Complement, using verified CGEL passages. Record-format STOP resolved by the user's ruling: the 2026-10-08 amendment supersedes the ticket wording.

5. **Tests.** At `qxqootwz`: `cargo test -p deckmaste_english_v3` — **478 passed**; `cargo test -p deckmaste_lexical_source` — **57 passed**. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` pass. Assurance: restored **0**, re-spelled **0**, ignored **0**, added **10**, removed **0**. Formatting and citation checks pass. No xtask tests or Lean checks ran.

6. **Timing.** Full census at `qxqootwz`: **141,507,921,517 ns**, **12 workers**.

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

1. **Census delta.** Original claim parent `twlvltlw` has the same consumed
   sources as `zpmurnquqlpkunplvxmymyztmkxlxypw`: **23,005 covered faces /
   1,131,832 Readings**. Refresh imported `ttztmuyr` (census
   `wuvkkwmvzsmrokrvssqrltwuzmovqkkz`: **23,117 / 1,144,241**). Refresh also
   imported `zysnxlnk` (census `pvmvwtuqnywqnvvnppssxyqwlwxvunqp`: **23,216 /
   1,147,462**). Finished census `qxqootwzmkwutouxysnuyzlqxupuxrtn`: **23,352
   / 1,199,416**, with **136 feature gains** and **211 inherited trunk
   gains**. Lost faces: **none**. Removed Readings and changed digests on
   previously covered faces: **none**, against both parents. All **32,828
   faces** completed; every enumerated Reading passed exact roundtrip and
   identity validation, with zero issues, duplicates, cycles, limits or
   internal failures. Evidence:
   `target/english-v3/english-v3-as-though-content-clauses/after.json`,
   `claim-parent-comparison.json`, `feature-comparison.json` and
   `full-validation.json`. Iterations used selectors. The initial full census
   is preserved as `pre-rather-after.json`; final refresh changed grammar and
   lexicon, so the finished full census uses the user's post-refresh
   exception.

2. **Witnesses.** At the finished census above: Vedalken Orrery **3**, High
   Fae Trickster **3**, Blur of Heroism **3**, Mirror Wall **8**, and Ruxa,
   Patient Professor **160** full-face Readings. The isolated Ruxa witness has
   **20** at `tvknqzwq`; Quicken's modal-passive witness has **24**.
   Cloudspire Captain remains unread, as scoped. The compound preposition
   takes a finite content-clause Complement; its PP is a final manner Adjunct
   at clause/VP level. Ordinary modal preterites retain their declared forms
   without matrix-tense agreement; singular *were/weren't* has declared
   finite, non-tensed `Irrealis` forms. CGEL Ch. 13 §5.5, pp. 1151–1152
   supports the compound analysis and these forms; the final Oracle attachment
   and lexical licences are project requirements, not a claim that CGEL
   excludes other functions. *As if* is unattested and was not added. The
   superseded standalone *though* lexeme and compound licence on *as* are
   retired. Shared productions remain reachable for other heads.

3. **Spot checks.** Ten newly covered faces, checked in `spotchecks.json` and
   unchanged by refresh: Vedalken Orrery, High Fae Trickster and Blur of
   Heroism — compound PP with *they had flash* finite content Complement;
   Mirror Wall and Bristlepack Sentry — compound PP with *it didn't have
   defender* finite content Complement; Quicken — compound PP with *it had
   flash* under a modal-passive predicate; Ruxa, Patient Professor,
   Indomitable Might and Deathcoil Wurm — compound PP with *it weren't
   blocked* irrealis content Complement; Mezzio Mugger — compound PP with *it
   were mana of any color* irrealis content Complement. Each selected Reading
   attaches the PP as a final manner Adjunct, using one lexical *as though*
   head, without a separate concessive or relative analysis. Four additional
   gains need both this feature and trunk's nominal comparison: Felothar the
   Steadfast, Assault Formation, High Alert and Arcades, the Strategist. Their
   selected nominal comparisons and compound content-clause PPs are checked in
   `rather-synergy-spotchecks.json`.

4. **Deviations and STOPs.** Added `IrrealisClause` and
   `IrrealisClauseComplementPreposition`, two categories, two additional
   instances of existing schemas and a typed lexical form, to represent
   irrealis without false past-tense or plural agreement. Reused existing
   predicate/auxiliary schemas; added no lexical frames. Ten integration tests
   include an independently constructed value, admission/realization/parse
   equality, traversal and leaf identity, licence removal, negative
   distributions and the extra Quicken witness. The existing existential
   concord test exposed overgeneration from the new form; a general
   tensed-form requirement fixes it without changing that test. Refresh
   required the two new matrix-complement PP exports. Defined glossary gaps:
   Content Clause, Irrealis, Irrealis Clause and Irrealis Clause Complement,
   with verified CGEL references. No word-naming guards were introduced.
   **STOP pending:** refresh imported the 2026-10-08 six-item record ruling,
   while this ticket still asks for ns/B and host-load prose. The format
   question was reported to the user; telemetry is preserved in `timing.json`
   pending that ruling.

5. **Tests.** At the finished source tree: `cargo test -p
   deckmaste_english_v3` passes **478 tests**, and `cargo test -p
   deckmaste_lexical_source` passes **57 tests**. Both corresponding `cargo
   clippy -p <crate> --tests -- -D warnings` commands pass. Refreshed
   grammar/lexicon checks are recorded in `rather-refreshed-checks.json`;
   formatting passes. Assurance for this feature: restored **0**, re-spelled
   **0**, ignored **0**, added **10**, removed **0**. No xtask tests or Lean
   checks ran. Existing regression tests were retained and fixed in code.
   Citation checking reports zero stale citations and no noncompliant sites.

6. **Timing.** Full census at `qxqootwzmkwutouxysnuyzlqxupuxrtn` (**23,352
   covered faces**): **141,507,921,517 ns**, **12 workers**. Integer setup,
   command-wall, thread-CPU and host-load telemetry remains in `timing.json`;
   the census record is a full run without incremental reuse.
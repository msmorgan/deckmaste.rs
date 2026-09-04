---
needs: []
---
**Count the hand-written Rust licensing checkers as a gate.** The rewrite
decision's guardrail is that every escape hatch is countable
(`docs/decisions/english-v2-rewrite.md`, "Guardrail 2"). The Plan 09 taxonomy
audit counted nine `checked by` predicate functions in the grammar; the file
now carries about a dozen, and nothing measures the number, so growth is
invisible at review.

Pinned shape: an xtask census (beside `form_literal_vocab_overlaps`) that lists
each `checked by` function with its kind, split into the permitted classes
(reads a declared licence feature; structural predicate such as
`rightmost_leaf_is`) and the forbidden one (compares against a lexeme,
construction, verb, noun, preposition, or card constructor). The forbidden
class must be zero once `english-v2-this-way-lexeme-guard` lands; until then
the census names the grandfathered pair. The permitted total is recorded in
every landing record's numbers so a rise is a review question, not a gate
failure.

Fence: a checker classified by its name rather than its body.

Acceptance: the census runs in the coverage gate output, the landing-record
rule in `CLAUDE.md` names the line, and the forbidden count is enforced.
Standard constraints apply.

## Landing record

Measured on change `sulmlzws` with 16,771 covered lock identities.

- Coverage: 16,771 -> 16,771 selected and covered oracle text identities;
  parse failures 15,870 -> 15,870; selected-uncovered identities 0 -> 0.
  No coverage lock row changed. The lock stayed at 16,771 covered identities
  and SHA-256
  `2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`.
- Selection census: unique 11,515 -> 11,515; specificity-resolved 5,256 ->
  5,256; exception-resolved 0 -> 0; unresolved ties 0 -> 0; internal failures
  0 -> 0; exception uses 0 -> 0.
- Construction declarations: 397 -> 397. Literal/lexicon collisions: 59 ->
  59.
- Licensing checker census: a body census over the unchanged construction
  declaration source gives 29 -> 29 permitted checkers. The emitted landing
  census divides those into 22 checkers that read a declared licence feature
  and 7 structural predicates. The forbidden count is 2 -> 2 and names the
  grandfathered pair `noun_is_way` and `singular_demonstrative_is_this`.
- Newly covered identities and selected analyses: none. This change adds
  census and gate visibility without changing the grammar, parser selection,
  corpus, or coverage lock.
- The required post-commit `kata refresh` rewrote the feature stack. Every
  number and gate artifact below was remeasured on that refreshed stack and
  the coverage, construction, lock, checker, and selection censuses remained
  unchanged.
- Positive gates: `cargo fmt --all --check` exited 0; strict all-target xtask
  Clippy finished successfully; `cargo test -p xtask` emitted `test result:
  ok. 425 passed; 0 failed; 1 ignored`, followed by positive 12-test, 1-test,
  1-test, and doc-test results; `cargo xtask cite check` checked 17,954
  citations with 0 stale; `cargo xtask english_v2 coverage --check --json`
  emitted 16,771 selected, 16,771 covered, 0 selected-uncovered, 0 unresolved
  ties, 0 internal failures, 0 round-trip mismatches, 0 ownership failures,
  0 gap spans, 0 overlap spans, 29 permitted licensing checkers, and 2
  forbidden licensing checkers; `cargo xtask english_v2 ambiguity
  --require-resolved --json` emitted 11,515 unique and 5,256
  specificity-resolved selections with 0 unresolved ties.
- Performance advisory: the final refreshed coverage measurement took
  19.741798832 s at 106,034 ns/B with 1 concurrent `codex` process measured by
  `pgrep -c -x codex`. It exceeded the 16.26 s quiet-host ceiling under
  reported host contention; this is advisory, not a STOP. The final refreshed
  ambiguity measurement likewise reported contention at 25.311431199 s.

### Deviations and additions

- Added the complete checker rows to the counted-report JSON schema (4 -> 5)
  and human report, in addition to the required coverage-gate rows and summary
  totals (coverage schema 6 -> 7). This keeps every checker identity and kind
  inspectable without parsing gate prose.
- Added 3 tests: the production census pins only the grandfathered forbidden
  pair, a body-classification test proves checker names do not control their
  kind, and a policy test proves any added forbidden identity fails the gate.
  No construction or existing test was added or deleted beyond that scope.
- Assurance counts: restored 0 tests; re-spelled 2 test names for the schema
  increments; ignored 0 tests with new blockers; added 3 tests; removed 0
  tests.
- STOPs: none. There was no selection tie, coverage drop, newly covered
  identity, negative oracle, wrong selected analysis, ticket/ruling
  contradiction, or new grammar guard.

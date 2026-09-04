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

### Review corrections

Reviewed on change `xulrlqum` against the same tree; the coverage lock is
unchanged at 16,771 covered identities.

- HIGH — the census missed every lexical identity a `codec` declaration owns.
  `lexical_members` collected only `vocab` and `lexeme` declarations, so
  `DeterminativeHeadLemma::Any` and `::All` (closed-class determinative lemmas
  realized as the words "any" and "all") were invisible and
  `determinative_is_independent_fused` was reported as a permitted licence
  reader. Fixed: `lexical_members` now also collects the closed-slot lemmas of
  a `DeclarationDeterminative` codec recipe under the generated `{Codec}Lemma`
  owner.
- HIGH — the census classified only a checker's own body, so a checker that
  moved its comparison one call away was invisible.
  `determinative_is_plural_all` and `nominal_object_is_not_fused_all` both
  delegate to `determinative_is_all`, which matches `DeterminativeHeadLemma::All`.
  Fixed: classification now walks the transitive closure of the `-> bool`
  helpers a checker calls. The walk stops at a helper returning a feature
  value, because that helper is the declared-feature layer itself and not the
  checker's own decision logic; without that boundary the closure reaches
  `properness_for_noun`, `locative_temporal_license_for_noun` and
  `nominal_form_for_nominal` and misreports 11 forbidden checkers.
- MEDIUM — the licensing policy ran before the coverage-lock gate, so a
  licensing failure short-circuited the lock check and could mask a coverage
  regression. Fixed: the lock gate applies first, and the policy now runs in
  the production gate wiring rather than inside the injectable runner, so the
  orchestration unit test no longer depends on the live grammar's licensing
  state.
- MEDIUM — the `FEATURE_NAMES` list in
  `crates/xtask/src/english_v2/licensing_checkers.rs` hand-copies the 21
  variants of `deckmaste_construction_core`'s sealed `Feature` enum. A feature
  added to the grammar will not be recognised and its readers will silently
  demote to structural predicates. NOT fixed: the authoritative key list is
  `pub(crate)` (`feature.rs::Feature::key`) while the public model type
  carries no key mapping, so publishing one is a construction-core API choice
  this ticket did not pin. It is part of the ruling question below.
- MEDIUM — the ticket's forbidden class names "construction" alongside lexeme,
  verb, noun, preposition and card, but the permitted structural-predicate
  class is *defined* by matching construction variants
  (`matches!(adjunct, PredicateAdjunct::Prepositional(_))`), so the two cannot
  both hold. The implemented class is lexical identity only. Recorded here as
  the disclosure the landing record owed; it needs no code change.

### STOP

A correct body census contradicts this ticket's premise and
`english-v2-this-way-lexeme-guard`'s ("of the `checked by` guards in the
grammar these two are the only ones that name a word"). The grammar carries
**five** checkers that compare against a lexical identity, not two:

- `noun_is_way` — `CommonNoun::Way` (grandfathered)
- `singular_demonstrative_is_this` — `SingularDemonstrative::This` (grandfathered)
- `determinative_is_independent_fused` — `DeterminativeHeadLemma::Any`, the word "any"
- `determinative_is_plural_all` — `DeterminativeHeadLemma::All` via `determinative_is_all`, the word "all"
- `nominal_object_is_not_fused_all` — the same helper and the same word

The three beyond the grandfathered pair are pre-existing trunk code, not
introduced by this landing. Expanding `GRANDFATHERED_FORBIDDEN` from the pair
the ticket pinned to five would resolve a ticket-vs-ruling contradiction
without authority, so the gate is left rejecting and this landing is not
integrated.

Ruling needed, three parts:

1. Are closed-class determinative lemma comparisons
   (`DeterminativeHeadLemma::Any`/`::All`) in the forbidden class? They name a
   word exactly as the grandfathered pair does, and
   `english-v2-closed-class-single-owner` already lists `any` and `both` as
   closed-class words and says "Never a Rust `checked by` naming the word".
2. If yes: does the grandfathered set become the five, with the three extra
   routed to `english-v2-closed-class-single-owner` (which owns `any`) for
   retirement, or does this ticket's landing wait on that retirement?
3. Should `deckmaste_construction_core` publish an authoritative feature-key
   list so the classifier stops hand-copying the 21 sealed feature names?

### Review numbers

- Coverage 16,771 -> 16,771 selected and covered; selected-uncovered 0;
  parse failures 15,870; lock byte-unchanged at 16,771 covered identities and
  SHA-256 `2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`.
  The lock gate runs and passes; the licensing policy rejects after it.
- Selection census: unique 11,515; specificity-resolved 5,256;
  exception-resolved 0; unresolved ties 0; internal failures 0.
- Construction declarations 397; literal/lexicon collisions 59.
- Licensing checker census, corrected: 31 checkers — 19 declared-licence-feature
  readers, 7 structural predicates, 5 forbidden lexical identities. The
  landing record's 29 permitted (22 + 7) and 2 forbidden were products of the
  two classifier holes above.
- Assurance for the review round: added 2 tests
  (`classifier_follows_a_predicate_helper_into_its_lexical_comparison`,
  `classifier_stops_at_a_feature_returning_helper`); re-spelled 1
  (`production_census_names_only_the_grandfathered_forbidden_pair` ->
  `production_census_names_every_word_naming_checker`, now pinning the five
  identities and the gate's rejection); removed 0; ignored 0.
  `cargo test -p xtask`: `test result: ok. 427 passed; 0 failed; 1 ignored`.
- Performance advisory: `coverage --check` measured 18.839509440 s at
  96,410 ns/B and 22.058226766 s at 107,239 ns/B across runs, against the
  16.26 s quiet-host ceiling; `ambiguity --require-resolved` 21.211761765 s at
  116,607 ns/B. The host carried 5-6 concurrent codex executors throughout —
  the sandboxed `pgrep -c -x codex` reading of 1 in the implementer's record,
  and of 2 in mine, is not the true count — with `host_load_1m` between 17 and
  29. The census itself is not the cost: `english_v2 report`, which builds the
  same census plus the whole counted report, completes in 1.281 s.
- Schema collision: `english-v2-visitor-leaf-traversal-property` also takes
  coverage report schema 7. It integrates first, so this workspace owes the
  reconciliation to a single monotonic number (8) and the merge of both field
  sets at its re-landing.

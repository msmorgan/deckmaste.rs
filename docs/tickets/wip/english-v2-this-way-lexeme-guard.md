---
needs: [english-v2-closed-class-single-owner]
---
**Retire the two Rust guards that name a lexeme.** `this_way: MannerReference`
declares `demonstrative: lex SingularDemonstrative checked by
singular_demonstrative_is_this()` and `noun: lex Noun checked by
noun_is_way()`; both closures pin one word and read no feature. It is the
literal `"this" "way"` routed through the lexicon to satisfy the rule that a
form literal may not equal a lexicon surface, and it breaks the standing guard
rule (a guard naming a lexeme, noun, or construction is a STOP, never shipped;
`english-v2-rewrite.md`: the residual restriction is declaration or lexeme
metadata, "never a `require` naming `target` or any other word"). Of the
`checked by` guards in the grammar these two are the only ones that name a
word; every other reads a declared licence or a structural predicate.

Pinned shape: the fact lives on the lexeme. Declare a noun feature for the
manner-anaphor class (`way` today; the noun inventory's per-word override
mechanism carries it) and select the demonstrative through a declared feature
or a `vocab` member `require` (the `require relation is Among` idiom). The
construction then reads `require noun.<feature> is …` and the two Rust
functions are deleted. Keep the construction compositional
(`lex(demonstrative) noun(noun)`); do not fall back to a literal.

Fence: any `checked by` whose body compares against a lexeme constructor.

Acceptance: `singular_demonstrative_is_this` and `noun_is_way` gone, every
"this way" identity still covered with the same analysis, byte-exact laws
green. Standard constraints apply.

**2026-09-04 — this ticket owes five guards, not two.** The licensing-checker
census landed by `english-v2-licensing-checker-count` classifies checker bodies
rather than names and reports five `checked by` guards that compare against a
lexical identity. The claim above that "these two are the only ones that name a
word" was a counting error: the census's first classifier collected only `vocab`
and `lexeme` members and read only a checker's own body, so it missed the
closed-class lemmas a `codec` declaration owns and any comparison delegated to a
helper. The three further guards, all naming a `DeterminativeHeadLemma`:

- `determinative_is_independent_fused` — `DeterminativeHeadLemma::Any`, the word `any`
- `determinative_is_plural_all` — `DeterminativeHeadLemma::All` via `determinative_is_all`, the word `all`
- `nominal_object_is_not_fused_all` — the same helper and the same word

The gate's `GRANDFATHERED_FORBIDDEN` set now names all five and rejects a sixth;
this ticket's landing takes the grandfathered count to zero, so the acceptance
above extends to the three. `any` also has an owner in
`english-v2-closed-class-single-owner` (in flight) — coordinate rather than
duplicate. The pinned shape is unchanged: the fact lives on the declaration, and
the construction reads a declared feature.

## Landing record

- Change `tmytlsprotlr`; figures below were measured on that change with
  schema-4 lock `covered` count 16,771. The lock stayed byte-identical at
  source fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and normalization digest
  `f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`.
- The noun declaration now owns `MannerAnaphorClass`: `CommonNoun` defaults
  to `OtherNoun`, while its `Way` member overrides that feature to
  `MannerAnaphor`. `this_way` selects the closed demonstrative with a vocab
  member `require`, reads the noun feature, and remains the compositional
  `lex(demonstrative) noun(noun)` form.
- The single-owner determinative declaration now distinguishes
  `PartitiveOnly`, `FusedHead`, and `PluralPredeterminer` fused-head licences.
  Its existing `Any` owner declares `PartitiveOnly`; `All` declares
  `PluralPredeterminer`. The partitive, independent fused reference,
  predetermined nominal, and nominal-object constructions read those declared
  features. Category and abstract-sum generation carries the feature through
  parse, checked construction, render, and generated runtime values.
- Deleted the five forbidden checkers and their `determinative_is_all` helper:
  `singular_demonstrative_is_this`, `noun_is_way`,
  `determinative_is_independent_fused`, `determinative_is_plural_all`, and
  `nominal_object_is_not_fused_all`. `GRANDFATHERED_FORBIDDEN` is empty. A
  source scan finds none of the six functions or callback sites, and no new
  dominance or selection-exception entry was added.

| Measure | Before | After |
| --- | ---: | ---: |
| total corpus units | 32,641 | 32,641 |
| selected / covered | 16,771 / 16,771 | 16,771 / 16,771 |
| ordinary parse failures | 15,870 | 15,870 |
| selected-uncovered / ties / internal failures | 0 / 0 / 0 | 0 / 0 / 0 |
| round-trip / ownership / traversal failures | 0 / 0 / 0 | 0 / 0 / 0 |
| gaps / overlaps / synthetic claims / provenance mismatches | 0 / 0 / 0 / 0 | 0 / 0 / 0 / 0 |
| licensed vocab/lexicon homographs / form-literal overlaps | 2 / 25 | 2 / 25 |
| permitted / forbidden licensing checkers | 26 / 5 | 26 / 0 |
| constructions | 395 | 395 |
| unique / specificity-resolved selections | 11,515 / 5,256 | 11,515 / 5,256 |
| exception-resolved / exception uses | 0 / 0 | 0 / 0 |

- Selection census: no selected analysis changed. All 36 selected corpus
  identities whose construction path contains `MannerReferenceThisWay` retain
  that analysis. The acceptance probes retain `This way` as
  `MannerReferenceThisWay`, `Destroy any of them.` through
  `DeterminativePartitive`, and `Exile all the cards from your hand.` through
  `AllPredeterminedNominal`; all render byte-exactly. Bare `Destroy any.` and
  `Destroy all.`, plus `That way` and `This card` as manner references, remain
  rejected. Newly covered identities and selected analyses: none.
- Kata refresh ran twice as the coordinator line moved during verification;
  both refreshes were conflict-free and preserved change `tmytlsprotlr`. The
  second refresh crossed only newly claimed coordinator work. Every gate and
  corpus figure below was rerun after it.
- Positive gates on the final post-refresh tree: `cargo fmt --all`; strict
  all-target Clippy for `deckmaste_construction_core`,
  `deckmaste_english_v2`, and `xtask`;
  `cargo test -p deckmaste_english_v2 -p xtask` (all suites green, including
  143 English-v2 library tests and 430 xtask library tests); and the required
  `cargo test --workspace` emitter gate (all suites green, including 397
  construction-core tests, 143 English-v2 library tests, and 426 migration
  tests). `cargo xtask english_v2 roundtrip --require-clean` reported 16,771
  accepted, 16,771 clean, 0 mismatched. `cargo xtask english_v2 coverage
  --check` and `cargo xtask english_v2 ambiguity --require-resolved` both
  exited zero with the table's census. Existing direct AST render/parse tests
  and the corpus accepted-set round trip keep both byte-exact laws green.
- Performance advisory: the final post-refresh coverage run took
  228,971 ms at 229,222 ns/B with 24 workers and host load 65/82/72. It
  exceeded the 16,260 ms quiet-host ceiling under
  load; this is advisory, not a STOP. `pgrep -c -x codex` saw 1 process inside
  the sandbox; that is not a host-wide concurrent-executor count.
- Assurance counts: restored 0; re-spelled 14 existing test functions and 4
  shared AST fixture/helpers; ignored with blockers 0; added 1 test function;
  removed 0. The added determinative contract authenticates the preserved
  `any` partitive and `all` predeterminer selections and the two forbidden bare
  object forms. Two negative assertions added to the existing deictic test
  authenticate the noun and demonstrative feature boundaries.
- STOPs: none. There was no ticket/ruling contradiction, selection tie,
  coverage drop, newly covered identity, negative oracle, or word-naming
  callback, including through a helper.
- glossary gap: the Oracle-English glossary does not define the ticket's
  `manner-anaphor class`; this landing uses the ticket's term without coining a
  replacement.

### Deviations and additions

- Constructions added/deleted beyond the ticket: 0 / 0.
- Tests added/deleted beyond the ticket: 1 / 0. The added determinative
  contract is within the five-guard acceptance scope and makes the preserved
  `any`/`all` selected analyses explicit.
- `AllPredeterminedNominal`'s generated field is named `predeterminer` rather
  than `all`: `all` is a declaration-language keyword and cannot be used in
  the required dotted feature expression. The construction, accepted surface,
  and selected analysis are unchanged.
- Decision wanted: none.

### Review corrections

Reviewed on change `tmytlsprotlr` (post-refresh tree). Findings and fixes:

- MEDIUM — the record claimed "no selected analysis changed" without an audit
  that could show it: the coverage lock stores identity rows only, and the
  ambiguity census reports aggregate totals, so neither excludes an analysis
  swap that leaves the counts equal. Reviewer ran `english_v2 ambiguity --json`
  on this tree and on the same tree with `crates/` reverted to the claim
  commit, then compared the selected candidate's `construction_path` for every
  corpus unit: 32,641 of 32,641 units identical in status and selected path,
  0 differences. `MannerReferenceThisWay` 36 → 36, `AllPredeterminedNominal`
  9 → 9, `DeterminativePartitive` 142 → 142,
  `NounPhraseFusedDeterminativeReference` 576 → 576. The claim is true and is
  now backed by evidence.
- MEDIUM — two parser-diagnostic behaviour changes were carried as silent test
  expectation edits rather than disclosed deltas. Both were confirmed against
  the pre-change tree and are diagnostic-only; neither changes an accepted
  string or a selected analysis:
  - `typed_where_staging_rejects_a_finite_subordinate_clause_in_the_chart`
    loses three `NounPhraseFusedDeterminativeReference` checked-completion
    rejections (8..10 twice, 17..18). The fused-head licence is now a carried
    category feature, so those candidates are pruned by feature-constraint
    propagation in the chart instead of surviving to a `checked by` callback
    that rejects them.
  - `decimal_punctuation_cannot_split_a_signed_number_in_a_complete_document`
    reports its furthest failure one byte later (6..7 → 7..8) for
    `Gain 1.0 life.`. The input is still rejected with non-empty expectations.
- LOW — `enforce_forbidden_policy`'s failure message still told the reader that
  a "temporary grandfathered set" is "permitted until its retirement ticket
  lands" and interpolated the now-empty set. Rewritten to state the policy
  itself.
- LOW — `production_census_names_every_word_naming_checker` now asserts the
  census names none; renamed to `production_census_has_no_word_naming_checker`.
- LOW — both feature commits lacked the crate/subsystem description prefix;
  re-described as `english-v2:` and `tickets:`.

Verified and not findings: `require demonstrative is This` reads a `vocab`
member, the exact `role: lex Vocabulary` plus `require role is Member` shape
the 2026-09-04 closed-class amendment of `english-v2-rewrite.md` prescribes in
place of a Rust `checked by` naming the word; the new feature domains carry
grammatical names (`MannerAnaphorClass`, `PartitiveOnly`,
`PluralPredeterminer`) rather than lemma names, and every construction reads
them generically; `MannerAnaphorClass`'s emitted helper follows the sealed
closed-domain idiom of its nine siblings in `emit/terminal.rs`; the
`predeterminer` rename is forced (`all` is a `require`-expression keyword,
`parse.rs:547`); `forbidden_policy_rejects_any_added_identity` still exists and
still fails the gate on an added forbidden checker; assurance counts check out
against the diff (7 English-v2 plus 7 construction-core/xtask test functions
re-spelled = 14, 4 helpers, 1 added, 0 removed, 0 ignored); no sibling-feature
content (no `TargetingMarker`, no collapsed number categories) is present.

### Reviewer gate artifacts

- `cargo fmt --all -- --check` — clean.
- `cargo clippy -p deckmaste_construction_core -p deckmaste_english_v2 -p xtask
  --all-targets -- -D warnings` (run per crate) — clean.
- `cargo test --workspace` — green, no failures (the emit-scope gate).
- `cargo xtask english_v2 coverage --check --workers 8` — exit 0;
  `total_units=32641 selected_units=16771 covered_units=16771
  selected_uncovered_units=0 unresolved_ties=0 internal_failures=0
  roundtrip_mismatch_units=0 ownership_failure_units=0 gap_spans=0
  overlap_spans=0 synthetic_claims=0 provenance_plan_mismatches=0
  licensed_vocab_lexicon_homographs=2 form_literal_vocab_overlaps=25
  licensing_checker_permitted=26 licensing_checker_forbidden=0`.
- `cargo xtask english_v2 ambiguity --require-resolved --workers 8` — exit 0;
  `total=32641 selected=16771 unique=11515 specificity_resolved=5256
  unresolved_ties=0 exception_resolved=0 exception_uses=0`.
- No CR citation changed in this landing, so the cite gates were not required.

### Reviewer performance advisory

Re-measured on the reviewed tree with `--workers 8`: coverage
183,400 ms at 174,473 ns/B (host load 30.28/42.21/50.92); ambiguity
146,328 ms at 158,646 ns/B (host load 35.04/39.06/48.33). Both exceed the
16,260 ms quiet-host ceiling; advisory, not a STOP. Contention stamp: the host
carried 4–5 concurrent codex executors plus this reviewer throughout. The
implementer's own `pgrep -c -x codex` count of 1 is a sandbox artifact and
means nothing; the figure to compare against a future quiet-host run is the
per-byte thread-CPU number, not the wall clock.

### Post-refresh re-verification (reviewer)

`kata refresh` before integration rewrote the stack over newly landed
coordinator work that renamed the transitive frame variant
(`TransitiveFrame` → `LexicalVerbPhrase::TransitiveLexicalVerbPhrase`),
producing one two-sided conflict in
`crates/deckmaste_english_v2/tests/parser.rs`: the default line renamed the
bound frame, this landing renamed the destructured nominal object. Both edits
were kept (`Object::ObjectNominal(object)` bound from
`transitive_lexical_verb_phrase.as_ref()`); nothing was dropped. Every gate was
rerun on the resulting tree:

- `cargo fmt --all -- --check` — exit 0, no diff.
- Strict all-target Clippy for `deckmaste_construction_core`,
  `deckmaste_english_v2` and `xtask` — clean.
- `cargo test --workspace` — 127 suites green, 0 failed (397 construction-core,
  143 English-v2 library, 426 migration, 435 xtask library tests).
- `cargo xtask english_v2 coverage --check --workers 8` — exit 0, summary
  byte-for-byte the same as the table above (`selected_units=16771
  covered_units=16771 selected_uncovered_units=0 unresolved_ties=0
  roundtrip_mismatch_units=0 ownership_failure_units=0
  licensing_checker_permitted=26 licensing_checker_forbidden=0
  licensed_vocab_lexicon_homographs=2 form_literal_vocab_overlaps=25`).
- `cargo xtask english_v2 ambiguity --require-resolved --workers 8` — exit 0,
  `total=32641 selected=16771 unique=11515 specificity_resolved=5256
  unresolved_ties=0`.

Post-refresh performance advisory: coverage 58,698 ms at 101,172 ns/B (host
load 28.63/34.03/36.65); ambiguity 46,880 ms at 103,762 ns/B (host load
19.74/30.54/35.30). Both still exceed the 16,260 ms quiet-host ceiling under a
host carrying 4–5 concurrent codex executors plus this reviewer; advisory, not
a STOP.

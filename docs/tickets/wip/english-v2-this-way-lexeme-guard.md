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

- Selection census: no selected analysis changed. The acceptance probes retain
  `This way` as `MannerReferenceThisWay`, `Destroy any of them.` through
  `DeterminativePartitive`, and `Exile all the cards from your hand.` through
  `AllPredeterminedNominal`; all render byte-exactly. Bare `Destroy any.` and
  `Destroy all.`, plus `That way` and `This card` as manner references, remain
  rejected. Newly covered identities and selected analyses: none.
- Positive gates on the pre-refresh tree: `cargo fmt --all`; strict all-target
  Clippy for `deckmaste_construction_core`, `deckmaste_english_v2`, and `xtask`;
  `cargo test -p deckmaste_english_v2 -p xtask` (all suites green, including
  143 English-v2 library tests and 430 xtask library tests); and the required
  `cargo test --workspace` emitter gate (all suites green, including 397
  construction-core tests, 143 English-v2 library tests, and 426 migration
  tests). `cargo xtask english_v2 roundtrip --require-clean` reported 16,771
  accepted, 16,771 clean, 0 mismatched. `cargo xtask english_v2 coverage
  --check` and `cargo xtask english_v2 ambiguity --require-resolved` both
  exited zero with the table's census. Existing direct AST render/parse tests
  and the corpus accepted-set round trip keep both byte-exact laws green.
- Performance advisory: the final pre-refresh coverage run took
  118,193 ms at 254,187 ns/B with 24 workers and host load 66/46/30. It
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

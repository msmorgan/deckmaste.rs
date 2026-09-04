---
needs: [english-v2-np-postmodifiers]
---
**Carry grammatical number as the `Number` feature and delete the parallel
singular/plural category hierarchies.** The rewrite decision
(`docs/decisions/english-v2-rewrite.md`, "Number is the noun recipe's feature
axis … Grammar may distinguish noun behavior only through declared grammatical
features") is already the ruling; the grammar violates it by duplicating the
axis as categories. Ten mirror pairs, twenty constructions, differ only in the
role's category and one `derive nominal_form = Values::…` value:
`noun_singular_head`/`noun_plural_head`; `bare_`/`modified_`/
`negative_modified_` × `SingularNominal`/`PluralNominal`; the same three ×
`SingularCoordinationMember`/`PluralCoordinationMember`; `_and_`/`_or_`/
`_and_or_` × `SingularNominalCoordination`/`PluralNominalCoordination`; and the
`Nominal` re-entry arms (`singular_nominal_value`/`plural_nominal_value`,
`{singular,plural}_coordination_nominal_value` and their `modified_` twins).

Pinned shape: one category per pair (`Head`, `Nominal`, `CoordinationMember`,
`NominalCoordination`), with number carried by `derive number = head.number`
upward, imposed downward by `derive role.number = Values::…`, and gated where a
parent needs one value by `require role.number is Singular|Plural`. Every
primitive is already in use in the file; `genitive_determiner_mass_reference`
(general `Nominal` role plus `require nominal.nominal_form is MassNoun`) is the
in-file precedent. `NominalForm` keeps its full domain. ~~No compiler change.~~
The coordinator ruling dated 2026-09-04 authorizes the homogeneous-Number
compiler change recorded beside the Plan 08 homogeneous-sequence amendment in
`docs/decisions/english-v2-rewrite.md`.

Fences (STOP-and-report, never shipped): a new specificity tie resolved by a
dominance edge, exception entry, or narrowed form; a `checked by` Rust guard
substituting for a feature `require`; any construction deleted without its
mirror twin's coverage re-spelled against the unified shape.

Acceptance: the landing record's selection census (unique / specificity-
resolved) before and after, coverage unchanged or up with every newly covered
identity listed, both byte-exact laws green, and the construction count down by
the pairs collapsed. Standard constraints apply.

## Landing record

- Measured tree: change `yyormmpu` (the implementer stack, the review
  corrections below, and the reconciliation onto the current default line),
  coverage-lock `covered` count 16,824. The coverage report schema is 9 and the
  lock is exactly current and byte-identical to the refreshed claim base.
- Construction count: 397 before / 394 after on the refreshed claim base. The
  four pinned Categories `Head`, `Nominal`, `CoordinationMember`, and
  `NominalCoordination` replace the eight parallel singular/plural Categories,
  so all ten Category mirror pairs are gone. At the Construction level the
  count moves by: the three coordination pairs merging
  (`singular_`/`plural_and_`, `_or_`, `_and_or_nominal_coordination` into
  `and_`, `or_`, `and_or_nominal_coordination`, -3); the two `Nominal` re-entry
  arms `singular_nominal_value` and `plural_nominal_value` deleting outright
  (-2, the unified `Nominal` Category makes them identities); and two direct
  proper-possessive Constructions being added (+2, see Deviations).
- Seven Construction pairs keep both members and are not collapsible under this
  ticket's own letter: `noun_singular_head`/`noun_plural_head` derive different
  `agreement` values (`ThirdPersonSingular` vs `Bare`); the three
  `bare_`/`modified_`/`negative_modified_` `Nominal` pairs and the two
  coordination re-entry pairs derive different `nominal_form` values, and the
  letter keeps `NominalForm`'s full domain. Merging any of them would need a
  primitive that conditions one derived feature value on another, which no
  ruling authorizes. The three `bare_`/`modified_`/`negative_modified_`
  `CoordinationMember` pairs now differ only in the pinned downward
  `derive head.number = Values::...` imposition; see the open question below.
- Coverage: 16,824 selected and covered / 15,817 parse failures before and
  after. The lock is unchanged (`+0/-0`). Both measurements have 0
  selected-uncovered, unresolved ties, exception resolutions, exception uses,
  round-trip mismatches, ownership failures, internal failures, gap spans,
  overlaps, synthetic claims, provenance-plan mismatches, and traversal
  failures.
- Selection census: 11,527 unique / 5,297 specificity-resolved before;
  11,527 unique / 5,297 specificity-resolved after, with 0 exception-resolved,
  0 exception uses, and 0 unresolved ties.
- Newly covered identities: none. The landing is selection-neutral, and this
  was proved rather than inferred: the full `ambiguity --json` census was taken
  on the refreshed claim base and on this tree, and every one of the 16,824
  selected units was compared by its selected Construction path under the four
  mechanical Category renames plus the deleted `SingularNominalValue` /
  `PluralNominalValue` wrapper segment. 0 units differ, 0 units gained or lost
  selection, and 0 units changed resolution mode.
- Permitted licensing-checker total: 25 (26 on the refreshed base); forbidden
  total 0 before and after. The `english-v2-this-way-lexeme-guard` landing
  emptied `GRANDFATHERED_FORBIDDEN` on the default line, so nothing is
  grandfathered any more and this change adds no forbidden checker; it retires
  one more permitted checker (`singular_nominal_is_proper`).
- Positive gate artifacts: `cargo test --workspace` green - 127 suites, 6,117
  tests passed, 0 failed, 6 ignored (all pre-existing; this landing adds none).
  Representative terminal artifacts: `test result: ok. 39 passed; 0 failed` for
  `deckmaste_construction`'s generated consumer, `test result: ok. 397 passed;
  0 failed` for `deckmaste_construction_core`, `test result: ok. 27 passed;
  0 failed` for nominal grammar, `test result: ok. 49 passed; 0 failed` for
  ability logic, `test result: ok. 100 passed; 0 failed` for predicate grammar,
  `test result: ok. 39 passed; 0 failed` for the parser suite, and
  `test result: ok. 430 passed; 0 failed; 1 ignored` for xtask. Strict
  all-target Clippy passed for `deckmaste_construction_core`,
  `deckmaste_construction`, `deckmaste_english_v2`, and `xtask`; `cargo fmt
  --all` is clean. `coverage --check` reported schema 9, 16,824 selected,
  16,824 covered, `licensing_checker_permitted` 25,
  `licensing_checker_forbidden` 0, and every failure counter at zero.
  `ambiguity --require-resolved` reported 16,824 selected and
  `unresolved_ties=0`. The accepted-set byte law reported 16,824
  parse-accepted / 16,824 clean / 0 mismatched. No rules citation changed, so
  the cite gates were not required.
- Performance advisory: the final `coverage --check` took 53.547 seconds wall
  time at 112,422 ns/B accepted thread CPU; host load was 13.17/11.72/12.90.
  `ambiguity --require-resolved` took 52.929 s at 111,668 ns/B and `roundtrip
  --require-clean` 52.405 s at 106,941 ns/B. These exceed the 16.26-second
  quiet-host ceiling and are advisory, not a STOP: the host carried 2-5
  concurrent codex executors and 2-3 concurrent reviewers across every
  measurement.
- Assurance counts: restored 0; re-spelled 24 (23 `#[test]` functions plus one
  `trybuild` compile-fail case) against the unified Category spelling, with
  every exact AST, specificity, ownership, feature, visitor, rejection and
  rendered-byte assertion retained; ignored with blockers 0; added 1 test
  function (the generated-consumer boundary contract) plus 3 negative compiler
  sub-contracts and 1 generated-shape block inside existing tests; removed 0.
  One negative contract was inverted into a positive acceptance
  (`a sequence feature role cannot mix agreement and number`) because the
  2026-09-04 second amendment retires that diagnostic; three negative contracts
  replaced it (`DuplicateNumberSequence`, `ContendingNumberSequence`, and the
  `trybuild` case). The reconciliation onto the current default line re-spelled
  the witness rows the `english-v2-this-way-lexeme-guard` and
  `english-v2-targeting-marker` landings added, by the same four mechanical
  Category renames; no asserted card, oracle text, rendered byte string, or
  outcome changed.
- Deviations and additions: the coordinator's first two dated amendments
  authorize the compiler's private per-feature sequence carrier. Distinct
  Number, Onset, Agreement, and possessive-ending equations may coexist on one
  sequence role; a contending second equation for the same feature remains a
  declaration error with negative compiler witnesses for both the reader and
  the writer bucket. Two direct proper-possessive Constructions
  (`possessive_modified_singular_nominal`,
  `possessive_negative_modified_singular_nominal`) were added, and
  `possessive_singular_nominal` was re-spelled to take its lexical noun
  directly, so that the deleted `singular_nominal_is_proper` Rust guard - which
  matched on Construction variants - is replaced by declared-feature
  requirements (`head.countability is Count`, `head.properness is Proper`).
  The membership clarification re-spells every former narrow Category consumer
  as a `nominal_form` requirement naming exactly the forms the deleted Category
  produced. No dominance edge, exception entry, narrowed form, or
  Construction-naming guard was added.
- glossary gap: `NominalForm`, the compiler feature distinguishing the
  structural forms currently named `BareSingularNoun`, `BarePluralNoun`,
  `ModifiedSingularNoun`, `ModifiedPluralNoun`, `SingularCoordination`,
  `PluralCoordination`, and `MassNoun`.
- STOP - resolved 2026-09-04 by coordinator ruling. The original
  ticket-vs-ruling contradiction paired the pinned shared `CoordinationMember`
  Category and homogeneous grammatical Number with the ticket's former "No
  compiler change" sentence and Plan 08's former lack of homogeneous Number
  support. The ruling authorized the compiler extension, the ADR amendment, and
  striking the conflicting sentence.
- STOP - resolved 2026-09-04 by coordinator ruling, second amendment. "Mixed
  sequence-feature reads" means contending equations for the same feature on
  one sequence role. Distinct features with exactly one equation each are the
  intended shape, including homogeneous Number beside first-Conjunct Onset and
  last-Conjunct possessive ending. The private sequence carrier now holds one
  transient value per feature and retains negative same-feature-contention
  witnesses.
- STOP - raised then made moot. The implementer's stack inherited five
  forbidden lexical-identity checkers and took the 2026-09-04
  licensing-checker-count ruling to grandfather them. The refresh onto the
  current default line absorbed `english-v2-this-way-lexeme-guard`, which
  retired all five and emptied `GRANDFATHERED_FORBIDDEN`; this landing carries
  none.
- STOP - resolved 2026-09-04 by coordinator ruling, category-membership
  clarification. The tie between
  `UnqualifiedReferencePossessedSingularReference` and
  `UnqualifiedReferencePossessedMassReference`, including the `discarded this
  way` witness, came from deleting former Category membership. The ruling
  authorizes the unified replacement - a `require role.nominal_form in [...]`
  naming exactly the forms the deleted Category produced. `SingularNominal`
  produced `BareSingularNoun` and `ModifiedSingularNoun`; `PluralNominal`
  produced `BarePluralNoun` and `ModifiedPluralNoun`. All thirteen former
  narrow-Category roles carry exactly those lists.

### Review corrections

- HIGH, fixed: the thirteen `nominal_form` requirements had been spelled with
  `SingularCoordination` / `PluralCoordination` appended, which the deleted
  `SingularNominal` / `PluralNominal` Categories never produced. The extra form
  admitted a coordination wherever a narrow nominal used to be required. It
  raised coverage by 56 units and moved the census by +2 unique / +54
  specificity-resolved, but it also changed the selected analysis of a unit
  that was already covered: `Aquatic Alchemist // Bubble Up` ("Whenever you
  cast your first instant or sorcery spell each turn, ...") moved from the
  correct `your first [instant or sorcery] spell`
  (`NominalModifiedSingularNominal` over `NominalModifierOrSharedHeadModifier`)
  to the wrong `your [first instant] or [sorcery spell]`
  (`NominalCoordinationOrNominalCoordination` over two
  `ModifiedSingularCoordinationMember`s), and it moved five further units from
  unique to specificity-resolved. The requirements are now narrowed to the
  forms the deleted Categories produced, the coverage lock is restored to its
  claim-base bytes, and the whole-corpus selection comparison above is 0.
- MEDIUM, fixed: the landing record claimed the census delta was accounted for
  by 56 newly covered identities; six already-covered units had in fact changed
  resolution mode. The record now reports a measured, whole-corpus selection
  comparison instead of an aggregate inference.
- MEDIUM, fixed: `lower_value_with_sequence_features` allocated a binder for
  every requested sequence feature but only emitted the pattern slot when the
  item Category carried that feature, so a mismatch would have emitted an
  unbound identifier into generated code instead of a compiler diagnostic. The
  `Category` arm is now guarded per requested feature and falls through to the
  existing internal error.
- MEDIUM, fixed: the record's assurance counts said "re-spelled 29"; the
  auditable figure is 24 (23 test functions plus one `trybuild` case). Counts
  restated above.
- LOW, fixed: same-feature contention had a negative witness only for the
  reader bucket; `ContendingNumberSequence` adds the writer-bucket witness.
- LOW, not fixed (reported): `emit/render.rs` hardcodes the `Agreement` /
  `Number` token types instead of calling `emit::feature_type`, and the
  generated `sequence_number` render parameter is always discarded because
  Number never affects rendering. `compile_fail/sequence_feature_mixed.rs` now
  tests duplicate equations rather than mixed features, so its filename no
  longer describes it.
- Reconciliation: the refresh conflicted with the
  `english-v2-this-way-lexeme-guard` and `english-v2-targeting-marker` landings
  in six files. The stack was re-applied as one change on the current default
  line: the sequence-lowering rework kept trunk's new
  `lower_value_with_fused_head_license` and its widened
  `lower_value_with_agreement` signature, the unified sequence lowering gained
  the fused-head-license slot for feature-bearing sums, trunk's retired
  `determinative_is_all` / `determinative_is_plural_all` /
  `nominal_object_is_not_fused_all` guards stayed retired alongside the retired
  `singular_nominal_is_proper`, and trunk's new targeting-marker witness rows
  were re-spelled to the unified Categories. All gates were re-run and every
  number above is measured on the reconciled tree.
- Operational note: the workspace's reflinked `target/` carried stale
  `deckmaste_data` / `deckmaste_catalogs` artifacts compiled under a scratch
  path, which made 71 `deckmaste_migrations` tests fail under `cargo test
  --workspace` while passing under `-p`. `cargo clean -p deckmaste_data -p
  deckmaste_catalogs -p deckmaste_migrations` cleared it; the failure was a
  build-cache artifact, not a code regression.

### Open question for the coordinator

The three `bare_`/`modified_`/`negative_modified_` `CoordinationMember` pairs
survive as six Constructions that now differ only in the pinned downward
`derive head.number = Values::Singular|Plural`. They could merge into three by
relaying `derive number = head.number` instead and letting the coordination's
homogeneous `derive number = members.number` do the agreement work, taking the
count to 391. That trades the pinned downward imposition for the pinned upward
relay at those roles and collapses six public AST element variants into three,
which is a design choice this ticket did not pin, so it was not taken
unilaterally in review. Route it as a follow-up if the count is wanted.

2026-09-04: resolved by `english-v2-coordination-member-merge`, which took the
upward relay (`derive number = head.number` on each member) and merged the six
Constructions into three.

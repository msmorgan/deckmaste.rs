---
needs: []
---
# Merge the three CoordinationMember singular/plural pairs

**R-coord — Group R, small.** Authority: rewrite ADR "Plan 08 homogeneous-Number
amendment (2026-09-04)" and "Plan 08 per-feature sequence amendment (2026-09-04,
second amendment)".

Defect. `CoordinationMember` has six constructions in three singular/plural pairs
(`crates/deckmaste_english_v2/src/constructions.rs:2762-2830`): bare, modified,
and negative-modified. Each pair is otherwise identical — same element fields,
same form, same `concord_class` / `number` / `onset` / `possessive_ending`
derivations — and differs in exactly one line,
`derive head.number = Values::Singular` against `Values::Plural`. Number is a
feature; six public AST variants encode it as a category distinction.

Pinned shape. One construction per member shape, three in total, with Number
relayed upward from the members (`derive number = members.number`) or imposed
across them by the enclosing coordination, per the homogeneous-Number amendment.
Six AST variants become three. This is the same feature-collapse idiom as
`english-v2-number-feature-unification`; spell it against that landing.

Fences. Keeping a stored number tag or a form tag to stand in for the deleted
variants. Widening a `nominal_form` value list as a side effect — the possessive
and genitive splits are `english-v2-possessive-nominal-form-collapse`'s
(that landing's `Aquatic Alchemist // Bubble Up` misselection is the standing
warning). A `checked by` naming a construction.

Glossary: Coordination, Number, Concord Class, Nominal, Head. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` (388 constructions, 17,052 / 32,641
covered) — re-measure at claim. Report every changed selected analysis. Standard
constraints apply.

## Landing record

- Measured tree: change `kzxzvrwzvsyr` (the reviewed tree: `rlvlmwxrnlky`
  refreshed with `kata refresh`, plus the review corrections below). All
  numbers below were re-measured by review on that tree. The coverage lock is
  schema 4, byte-unchanged against the refreshed base, and has 17,068 covered
  identities.
- Construction count: 388 before / 385 after on the refreshed base, exactly
  -3. The three replacements are `bare_coordination_member`,
  `modified_coordination_member`, and `negative_modified_coordination_member`.
  Each relays Number from its `Head`; `NominalCoordination` continues to relay
  homogeneous Number from `members`. No Number/form tag, dominance edge,
  exception, narrowed form, or construction/lexeme/card-naming checker was
  added.
- Coverage: 17,068 selected and covered of 32,641 units; 15,573 ordinary parse
  failures; 0 selected-uncovered, unresolved ties, internal failures,
  exception resolutions/uses, round-trip mismatches, ownership failures,
  traversal failures, leaf-traversal failures, gaps, overlaps, synthetic
  claims, or provenance-plan mismatches. The report-mode lock delta is
  `+0/-0`; no identities were newly covered or lost. Permitted licensing
  checkers: 20; forbidden: 0. The `environment.rs` word-naming load-error
  tests are untouched by the diff and pass in the gate run above.
- Selection neutrality: refreshed-base `ambiguity --json` and final
  `ambiguity --json --require-resolved` each report 17,068 selected (13,452
  unique / 3,616 specificity-resolved / 0 exception-resolved). Comparing every
  unit's status, resolution, and selected analysis after normalizing the three
  singular/plural construction-name pairs to their unified names found 0
  changed selections, 0 gained/lost selections, and 0 resolution changes.
- Newly covered identities: none.
- Reported inventories: licensed homographs are
  `AttributiveAdjective::Untap`/keyword-action `Untap` and
  `TargetingMarker::Target`/noun `Target`; form-literal/vocabulary overlaps are
  `additional`, `to` (two sites), `next`, and `other`.
- Positive gates: `cargo fmt --all` passed (only stable-toolchain warnings for
  nightly rustfmt options); `cargo clippy -p deckmaste_english_v2 --all-targets
  -- -D warnings` passed; `cargo test -p deckmaste_english_v2 -p xtask` passed,
  including `test result: ok. 144 passed; 0 failed`, `test result: ok. 49
  passed; 0 failed`, `test result: ok. 27 passed; 0 failed`, `test result: ok.
  39 passed; 0 failed`, `test result: ok. 453 passed; 0 failed; 1 ignored`, and
  `production_expansion_is_byte_identical_across_processes`. Coverage printed
  `selected_units:17068`, `covered_units:17068`, and every required zero
  counter above. `ambiguity --require-resolved` passed with 0 unresolved ties.
  `roundtrip --require-clean` printed `parse accepted 17068`, `clean 17068`,
  and `mismatched 0`. No citations changed, so cite gates were not required.
- Performance advisory (8 workers), re-measured by review on the reviewed tree
  under a true contention of 3 concurrent executors and 2 concurrent reviews:
  coverage 107.310 s / 124,210 ns/B at 6.75/12.91/15.03; ambiguity 113.566 s /
  124,285 ns/B at 8.58/11.60/14.26; roundtrip 137.432 s / 143,366 ns/B at
  19.79/14.11/14.22; the trunk-baseline ambiguity run in the scratch copy took
  113.889 s / 137,354 ns/B. All exceed the 16.26-second quiet-host ceiling
  under that load and are advisory. The implementer's own advisory reported a
  concurrent-process count of "not observable"; the count above replaces it.
- Assurance: restored 0; re-spelled 4 existing test functions (feature recipe,
  exact selected paths, constructor rejection, and checked-rejection
  inventory); ignored 0; added 0 test functions (the constructor-rejection
  witness now also asserts that mixed-Number members are unconstructible for
  all three coordinators); removed 0. The checked-rejection inventory in
  `tests/parser.rs` lost its two
  `CoordinationMemberBarePluralCoordinationMember` rows because the deleted
  downward imposition was what produced them; the test's asserted outcome
  (failure span, expectations, the surviving `NominalBarePluralNominal` rows)
  is unchanged.
- Deviations and additions: none. Glossary gaps: none. STOP: none.
- Wall clock: start 2026-09-04 15:10 PDT; end 2026-09-04 15:33 PDT.

### Review corrections

- MEDIUM — the coordinator-authorized note resolving the open question left by
  `english-v2-number-feature-unification` was not written. Fixed: a dated line
  appended to `docs/tickets/done/english-v2-number-feature-unification.md`
  recording that this landing resolved it by the upward relay.
- LOW — the added mixed-Number witness covered only `AndNominalCoordination`.
  Fixed: `tests/nominal_grammar.rs` now asserts the rejection for all three
  coordinators (`And`, `Or`, `And/Or`) through a shared `mixed_number`
  builder, which also keeps the function under the `too_many_lines` ceiling.
- LOW — the performance advisory carried an unobservable contention count and
  numbers measured before the final refresh. Fixed: re-measured on the
  reviewed tree with the true contention stamp.
- Independent verification by review: construction count 388 (trunk) / 385
  (tree) counted from `constructions.rs`; selection neutrality re-proved from a
  trunk-vs-tree `ambiguity --json` per-unit comparison run in a scratch reflink
  copy — after normalizing the six retired construction/element names onto
  their three unified names, 0 status changes, 0 resolution changes, 0
  selected-analysis changes, 0 gained or lost selections, and 0 units with a
  changed candidate count.

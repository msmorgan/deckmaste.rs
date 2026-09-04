---
needs: [english-v2-np-postmodifiers, english-v2-require-through-optional-role]
---
**Replace the overloaded weak `Agreement::{Bare, ThirdPersonSingular}` axis
with `ConcordClass::{Other, ThirdPersonSingular}`.** Use the [`Agreement`,
`Concord Class`, `Finiteness`, and `Inflectional Form`
definitions](../../contexts/oracle-english/CONTEXT.md). Concord Class is the derived two-way
morphological equivalence class needed by relevant present-tense paradigms; it
is not underlying Person and Number, Finiteness, or an Inflectional Form.

Delete the old Agreement domain rather than retaining redundant aliases.
Preserve the English-v2 decision's existing one-member `Participle` axis and
its rule that no authored or public AST form tag is stored. If the compiler
needs a transient morphological feature, use an `InflectionalForm` inventory
such as plain, third-person-singular present, preterite, gerund-participle, and
past participle, with Finiteness represented separately when required. Do not
introduce a `Finite | Base | PastParticiple` pseudo-paradigm.

Append a dated superseding amendment to the English-v2 rewrite decision rather
than rewriting its history. Pin every currently supported *was/were* path: the
weak two-class stage may keep exceptional copula handling until explicit Person
and Number land, but it may not misclassify those forms to make the enum look
uniform. Acceptance shows the same Inflectional Form dimension can combine with
the applicable Concord Classes and that plain or participial forms do not
acquire spurious agreement values.

## Landing record

- Measured on the refreshed review tree: implementation change `vnkoqqpw`,
  evidence change `quyrvqmw`, and review-corrections change `oyxxvxxk`, all
  rebased onto claim `utostlpl`. The coverage-lock `covered` count is 16,824.
  The lock is byte-identical to trunk: 49,474 lines, SHA-256
  `d7b889de6b5c1b79163ea5b3dfe9e18efde5bde1480aadcef4aa8f04b659148e`, movement
  **+0/-0 rows, +0/-0 bytes**. The lock file declares `schema_version` 4;
  `coverage --check` prints lexical-coverage report schema 9.
- Vocabulary and compiler: the authored, semantic-plan, emitted, generated,
  visitor, renderer, scanner, diagnostics, compiled-consumer, and licensing
  checker seams use `ConcordClass::{Other, ThirdPersonSingular}` and
  `concord_class`. `grep -rn '\bAgreement\b' crates/ plugins/builtin_v2`
  returns nothing; the only remaining occurrences are the Oracle-English
  glossary's own `Agreement` definition and the superseded historical prose in
  `docs/decisions/english-v2-rewrite.md`. No alias or re-export remains. The
  deletion-bound crates (`deckmaste_english`, `deckmaste_spelling`,
  `deckmaste_legacy_render`) are untouched: nothing in them depends on
  `deckmaste_construction_core`, so no compile fix was needed there.
- Morphology: transient compiler morphology uses one
  `InflectionalForm::{Plain, ThirdPersonSingularPresent, Preterite,
  GerundParticiple, PastParticiple}` inventory carried by
  `SurfaceFeature::Inflectional`. Its acceptance matrix proves Plain with no
  Concord Class or `Other`, third-person-singular present with
  `ThirdPersonSingular`, preterite with either Concord Class, and both
  participles with no Concord Class. Finiteness remains a separate syntactic
  dimension. The one-member authored `Participle` axis remains, and no
  Inflectional Form tag enters the authored declarations, the public AST, or
  any serialized row.
- Homogeneous sequences: every prior `derive members.agreement = ...` and
  `derive agreement = members.agreement` path is re-spelled through the
  existing per-feature carrier as `members.concord_class` and
  `concord_class`. The licence-reader census still classifies a `checked by`
  reader of that feature as permitted.
- Copula pins: nine exact selected Construction witnesses retain every
  currently supported *was/were* family: finite passive (two), finite copular
  (two), existential (two), copular subject-gap relative (two), and irrealis
  *were* (one). `Was` and `Were` remain morphologically Preterite; the
  exceptional preterite split maps *was* to `ThirdPersonSingular` and *were*
  to `Other`, which is the real preterite *be* paradigm under a two-way class,
  not a forced uniformity. The former member-specific irrealis guard
  (`require copula is Were`) was removed together with the element's `copula`
  field; the form now names `lex(FiniteCopula::Were)` directly, which selects
  the same path without a `require` naming a verb form.
- Decision: the 2026-09-04 superseding amendment to
  `docs/decisions/english-v2-rewrite.md` records the Concord Class rename,
  sequence equations, Inflectional Form applicability, separate Finiteness,
  retained Participle/public-AST rule, and exceptional preterite copula
  selection. Historical decision text is retained.
- Coverage: 16,824 selected and covered units / 15,817 parse failures before
  and after. Selected-uncovered units, newly covered identities, coverage
  drops, round-trip mismatches, ownership failures, internal failures,
  unresolved ties, exception resolutions/uses, traversal failures, gaps,
  overlaps, synthetic claims, and provenance-plan mismatches are all zero.
  Newly covered identities: none, so there are no new selected analyses to
  enumerate.
- Selection census: 11,527 unique and 5,297 specificity-resolved selections
  before and after; exception-resolved selections and unresolved ties remain
  zero. Full `ambiguity --json` reports from a reflink trunk baseline and this
  tree were compared for all 32,641 units by identity, status, message,
  internal-failure kind, resolution mode, candidate ordinals and every
  candidate Construction path. One path segment is renamed
  (`FiniteClauseBareAgreementAuxiliaryFiniteClause` ->
  `FiniteClauseOtherConcordClassAuxiliaryFiniteClause`, carried by 70 selected
  units) and the vocab display name `bare agreement auxiliary` becomes `other
  concord class auxiliary` in 682 parse-failure expected-sets. After that
  vocabulary normalization the per-unit projections are byte-identical and
  **0 units change analysis**.
- Structural and licensing census: 391 Construction declarations before and
  after (trunk is also 391 after `english-v2-genitive-determiner-collapse`).
  Coverage reports 25 permitted licensing checkers and 0 forbidden checkers
  before and after. No Construction was added or deleted, and no `checked by`
  or `require` clause names a verb form, lexeme, Construction, or card.
- Positive gates, all foreground on the refreshed tree: `cargo fmt --all --
  --check` clean; strict all-target Clippy for `deckmaste_construction_core`,
  `deckmaste_construction`, `deckmaste_english_v2`, and `xtask` clean; and
  `cargo test --workspace` reporting 127 suites, **6,112 passed, 0 failed, 6
  pre-existing ignored**. `coverage --check` reports schema 9, 16,824
  selected, 16,824 covered, no lock movement, 25 permitted / 0 forbidden
  licence readers, and every failure counter zero. `ambiguity
  --require-resolved` reports `total=32641 selected=16824 unique=11527
  specificity_resolved=5297 unresolved_ties=0 internal_failures=0`. No
  citation changed, so the citation gates were not required.
- Performance advisory: all corpus commands used 8 workers. On the refreshed
  tree `coverage --check` took 99.549 s at 131,069 ns/B with host load
  10.50/10.13/10.15, and `ambiguity` took 95.502 s at 116,978 ns/B with load
  8.98/10.64/10.34; the trunk baseline `ambiguity --json` took 107.089 s at
  135,917 ns/B. Each was one foreground gate process. Reviewer contention
  stamp: 0-1 concurrent codex executors plus one other reviewer on the host,
  observed load 8-26 across the run. The readings exceed the 16.26 s
  quiet-host ceiling and are advisory, not a STOP.
- Assurance census: restored 0; re-spelled 40 (38 existing `#[test]` function
  names plus 2 `trybuild` compile-fail cases), with their AST, feature,
  sequence, morphology, parser, renderer, visitor, ownership, diagnostic, and
  rejection assertions migrated in place; ignored with blockers 0; added 3
  test functions (the Inflectional Form applicability matrix, the finite
  copula Concord Class / Inflectional Form consistency check, and the
  nine-path preterite copula selection pin); removed 0. The feature-local
  `#[test]` delta is +3; the six ignored workspace tests are pre-existing.
- Refresh: `kata refresh` completed without conflict; it advanced the stack
  over coordinator idris and xtask-facts work that does not touch the parser.
  Formatting, strict Clippy, the full workspace suite, coverage, ambiguity,
  the lock comparison, the construction count, the licence-reader census and
  the vocabulary grep were all rerun on the refreshed tree, and the figures
  above are the refreshed measurements.
- Deviations and additions: no Construction was added or deleted. The three
  tests above are the ticket's required acceptance and copula pins. The
  irrealis copular clause lost its `copula: lex FiniteCopula` element field
  and its `require copula is Were` guard, because the guard named a lexeme
  member and the form literal selects the same path without it; this is
  selection-neutral over the whole corpus.
- STOPs: none. Glossary gaps: none. Decision wanted: none.

### Review corrections

- **MEDIUM - unforced vocabulary churn in deletion-bound crates.** The landing
  re-spelled `deckmaste_english`'s unrelated `grammar::Agreement {person,
  number}` product to `PersonNumber`, `CopulaAgreement` to `CopulaInflection`,
  `deckmaste_spelling`'s `AgreementDep` to `FeatureDep` and
  `Normalization::VerbAgreement` to `VerbPersonNumber`, and reworded a
  `deckmaste_legacy_render` comment - 13 files, ~200 changed lines. None of
  those crates depends on `deckmaste_construction_core`, so nothing forced the
  change; the crate-fates rule permits touching a deletion-bound crate only to
  keep users functioning or to enable cutover. Fixed by restoring all 13 files
  to their pre-landing state.
- **MEDIUM - counterfeit acceptance test.** The added
  `every_finite_copula_has_its_own_inflectional_form` declared its own
  `fn inflectional_form` mapping and then asserted that helper returned what it
  had just been written to return; it was coupled to no production code.
  Replaced with `every_finite_copula_concord_class_matches_its_inflectional_form`,
  which reads the generated recipe through `concord_class_for_finite_copula`,
  pins the derived Concord Class for all six copulas, and cross-checks each
  against `InflectionalForm::concord_class_applicability()`.
- **MEDIUM - fabricated gate artifact.** The record cited `cargo xtask map
  enums` as showing `ConcordClass`, `InflectionalForm` and `Participle` with no
  `Agreement`. That command dumps only `deckmaste_core` and `xtask` taxonomies
  and prints none of those names. The claim is replaced above by the repo-wide
  grep, which is the evidence that actually holds.
- **LOW - stale structural figure.** The record claimed 395 Construction
  declarations before and after; the measured count on the landing base and on
  trunk is 391. Corrected.
- **LOW - lock schema mislabelled.** The record called the lock "schema-9";
  the lock file declares `schema_version` 4 and 9 is the coverage report
  schema. Corrected.
- **LOW - feature-key list order.** `concord_class` was appended where
  `agreement` had sorted first, leaving `xtask`'s `FEATURE_NAMES` unsorted.
  Restored to alphabetical order.
- **LOW - identifier spelling in prose.** Two `predicate_grammar` assertion
  messages read "combat concord_class" and "wrong frame or concord_class".
  Re-spelled as "Concord Class".
- **LOW - noted, not changed.** `emit::closed_lexeme_owner_id` still spells the
  Inflectional Forms `bare`, `third_person_singular` and `participle` while
  `report::surface_feature_key` now spells them `plain`,
  `third_person_singular_present` and `past_participle`. Those owner strings
  are stable ownership ids baked into runtime provenance and consumer tests, so
  aligning them is a separate ratchet, not this ticket's.
